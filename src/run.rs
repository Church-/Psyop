extern crate regex;
mod process;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use process::Process;


fn readFile(filePath: &str) -> String {
    let mut tmpFile = File::open(filePath).expect("Filed failed to open.");
    let mut contents = String::new();
    tmpFile.read_to_string(&mut contents);
    contents
}

fn splitOnce<'a, 'b>(in_string: &'a str, pattern: &'b str) -> (&'a str, &'a str) {
    let mut splitter = in_string.splitn(2, pattern);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

fn getUser(line: &'static str) -> &'static str {
    let tmpLine = line.trim_left();
    let line = tmpLine.split(" ").collect::<Vec<_>>();
    let user = mapUsers(&line[0]);
    user.as_str()
}

fn mapUsers(uid: &'static str) -> String {
    let passwdFile = readFile("/etc/passwd");
    let mut user = "";
    let re = Regex::new(uid).unwrap();
    for i in passwdFile.lines() {
        let line = i.split(":");
        let tmpLine = line.collect::<Vec<&str>>();
        match re.is_match(tmpLine[2]) {
            true => user = tmpLine[0],
            false => continue,
        }
    }
    let mut tmpString = String::new();
    tmpString.push_str(user);
    tmpString
}

fn assignProcessStats(buffer: String) -> (isize, Process) {
    let mut name = "";
    let mut state = "";
    let mut pid = "";
    let mut mem: f64 = 0.0;
    let mut user = String::new();

    let tmpBuffer = (&buffer).lines();
    for i in tmpBuffer {
        let (first, second) = splitOnce(i, ":");
        match first {
            "name" => name = second,
            "state" => state = second,
            "tgid" => pid = second,
            "VmRSS" => mem = getProcessMemory(second),
            "uid" => user.push_str(getUser(second)),
            _ => continue,
        }
    }
    let tmpProcess = Process::new(name, state, pid, mem, &user, "");
    return (pid.parse::<isize>().unwrap(), tmpProcess);
}


fn getSystemMemory() -> f64 {
    let re = Regex::new(r"^MemTotal$").unwrap();
    let meminfo = readFile("/proc/meminfo");
    let mut systemMem = String::new();
    for i in meminfo.lines() {
        let (first, second) = splitOnce(i, ":");
        match re.is_match(first) {
            true => {
                let (tmpMem, _first) = splitOnce(second, "kB");
                tmpMem.trim();
                systemMem.push_str(tmpMem);
            }
            false => continue,
        }
    }
    systemMem.parse::<f64>().unwrap()
}

fn getProcessMemory(procMem: &str) -> f64 {
    let (tmpMem, _first) = splitOnce(procMem, "kB");
    let tmpMem = tmpMem.parse::<f64>().unwrap();
    let finalMem: f64 = ((tmpMem / getSystemMemory()) * 100.0);
    finalMem.trunc()
}



/*fn printStatusMap(processMap: BTreeMap<isize,Process>) {
	let tmpString = String::new();
	for (_tmp,processes) in processMap {
		tmpString.push(processes.toString());
		tmpString.push('\n');
	}
	println!("{}",tmpString);
}
*/

fn parseProc() -> Vec<String> {
    let path = Path::new("/proc");
    let mut folders: Vec<String> = Vec::new();
    let re = Regex::new(r"^\d+$").unwrap();
    let mut tmpPath = String::new();
    for entry in path.read_dir().expect("Failed to read folder contents.") {
        if let Ok(entry) = entry {
            tmpPath.clear();
            tmpPath.push_str(entry.path().to_str().unwrap());
            match re.is_match(&tmpPath) {
                true => folders.push(tmpPath.clone()),
                false => continue,
            }
        }
    }
    folders
}

fn run() {
    let mut processMap: BTreeMap<isize, Process> = BTreeMap::new();
    for i in parseProc() {
        let mut mutPath = PathBuf::from("/proc");
        mutPath.push(i);
        match mutPath.is_dir() {
            true => {
                mutPath.push("/status");
                let buffer = readFile(mutPath.to_str().unwrap());
                let (pid, tmpProcess) = assignProcessStats(buffer);
                processMap.insert(pid, tmpProcess);
            }
            false => continue,
        }
    }
}