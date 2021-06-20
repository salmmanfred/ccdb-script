pub struct Custom{
    pub name: Vec<String>,
    pub function: Vec<fn(Vec<String>) -> String>,
}
impl Custom{
    pub fn new() -> Custom{
        Custom{
            name: Vec::new(),
            function: Vec::new(),
        }
    }
    pub fn new_fn(&mut self, custom: fn(Vec<String>) -> String, name: &str){
        self.name.push(name.to_string());
        self.function.push(custom);
    }
    pub fn run_fn(&self, name: String, args: Vec<String>) -> String{
        let name = name.to_string();
        for x in 0..self.name.len(){
            if self.name[x] == name{
                let fsn = self.function[x];

                return fsn(args)
            }
        }
        panic!("no custom function")
    } 
}