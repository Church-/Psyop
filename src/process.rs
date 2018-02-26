pub struct Process {}
 
 
impl Process {
 
    fn new(&self, argList: Vec<String>) -> Process {
        let name = argList[0];
        let state = argList[1];
        let pid = argList[2];
        let mem = argList[3];
        let time = argList[4];
    }
 
 
    fn toString(&self) -> String {
       
    }
   
}
