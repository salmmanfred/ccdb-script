pub enum FnType{
    Output(fn(Vec<String>) -> String),
    Nout(fn(Vec<String>)),
}

pub struct Custom{
    pub name: Vec<String>,
    pub function: Vec<FnType>,
}
impl Custom{
    pub fn new() -> Custom{
        Custom{
            name: Vec::new(),
            function: Vec::new(),
        }
    }
    pub fn new_fn(&mut self, custom: FnType, name: &str){
        self.name.push(name.to_string());
        self.function.push(custom);
    }
    pub fn run_fn(&self, name: String, args: Vec<String>) -> Option<String>{
        let name = name.to_string();
        for x in 0..self.name.len(){
            if self.name[x] == name{
               
                match self.function[x]{
                    FnType::Output (a) =>{
                        return Some(a(args))
                    }
                    FnType::Nout (a) =>{
                        a(args);
                        return None
                    }
                }

          
            }
        }
        panic!("no custom function")
    } 
}