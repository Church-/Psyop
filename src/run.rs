extern crate regex;
mod process;
use std::str::Lines;
use regex::Regex;
use std::path::{Path,PathBuf};
use std::collections::BTreeMap;
use std::fs::File;
use process::Process;


fn readFile(filePath: String)  -> Lines<'a> {
	let tmpFile = File::open(filePath);
	let contents = String::new();
	tmpFile.read_to_string(&mut contents);
	//let mut lines: Lines = contents.lines()
	//Lines
	contents.lines()
}

fn splitOnce(in_string: &str, pattern: &str) -> (&'static str, &'static str) {
    let mut splitter = in_string.splitn(2, pattern);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}


fn assignProcessStats(buffer: Lines) -> (isize,Process) {
	let tmpVec: Vec<&str> = Vec::new();
	for i in buffer {
		let (first,second) = splitOnce(i, ":"); 
		match first {
			"name" => tmpVec.push(second),
			"state" => tmpVec.push(second),
			"tgid" => tmpVec.push(second),
			"VmRSS" => tmpVec.push(second),
			"" => tmpVec.push(second),
			_ => continue
		}
	}
		
}

fn getSystemMemory() {
	let meminfo = readFile("/proc/meminfo");
	let systemMem = String::new();
		for i in meminfo {
			let (first,second) = splitOnce(i, ":"); 
			match first {
				"name" => (systemMem,) = splitOnce(first, "kB"),
				_ => continue
			}
		}
	systemMem.parse::<isize>()
}


fn printStatusMap(processMap: BTreeMap<isize,Process>) {
	let tmpString = String::new();
	for (_,processes) in processMap {
		tmpString.push(processes::toString());
		tmpString.push("\n");
	}
	println!("{}",tmpString);
}

fn parseProc() -> Vec<String> {
	let path = Path::new("/proc");
	let mut folders: Vec<String> = Vec::new();
	let re = Regex::new(r"^\d+$");
	let mut tmpPath = "";
	for entry in path.read_dir().expect("Failed to read folder contents.") {
		if let Ok(entry) = entry {
		        tmpPath = entry.path().to_str();
			}
		match re.is_match(tmpPath) {
			true => folders.push(tmpPath),
			false => continue,
		}
	}	
	folders
}

fn run() {
	let mut processMap: BTreeMap<isize,Process> = BTreeMap::new();
	for i in parseProc() {
		let mut mutPath = PathBuf::new("/proc");
		mutPath.push(i);
		match mutPath.is_dir() {
			true => {
			    mutPath.push("/status");
		    	let mut buffer = readFile(mutPath.to_str().unwrap());
				let mut process = assignProcessStats(buffer);
				processMap.insert(i,process);
		        }
		    false => continue,
		}
	}
}
