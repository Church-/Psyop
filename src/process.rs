pub struct Process {
    cmdName: &'static str,
    state: &'static str,
    pid: &'static str,
    mem: f64,
    time: &'static str,
    user: &'static str,
}


impl Process {
    pub fn new(
        name: &'static str,
        state: &'static str,
        pid: &'static str,
        mem: f64,
        time: &'static str,
        user: &'static str,
    ) -> Process {

        Process {
            cmdName: name,
            state: state,
            pid: pid,
            mem: mem,
            time: time,
            user: user,
        }
    }

    pub fn toString(&self) -> String {
        let mut tmpStr = String::new();
        tmpStr.push_str(self.pid);
        tmpStr.push_str(self.user);
        tmpStr.push_str(self.time);
        tmpStr.push_str(&self.mem.to_string());
        tmpStr.push_str(self.cmdName);

        return tmpStr;
    }
}
