
use std::mem;
use crate::_mem;

#[derive(Clone)]
pub struct Var {
    pub var: Vec<String>,
    pub var_name: Vec<String>,
    pub max_mem: u64,
}
impl Var {
    pub fn new() -> Var {
        Var {
            var: Vec::new(),
            var_name: Vec::new(),
            max_mem:1,
        }
    }
    //make a new Var thats a string the only 2 variables in this language
    pub fn new_var(&mut self, name: &str, value: &str) {
        

        self.var.push(value.to_string());
        self.var_name.push(name.to_string());
        self._chk_mem();

    }
    // get a Var from memory #MEM:var
    pub fn get_var(&mut self, name: String) -> String {
        self._chk_mem();
        let x = self.var_name.iter().position(|x| *x == name);
        match x {
            Some(a) => {
                return self.var[a].clone();
            }
            None => {
                panic!("no var {}", name);
            }
        }
    }
    //update the variable
    pub fn _up_var(&mut self, name: &str, new_val: &str) {
        let x = self.var_name.iter().position(|x| *x == name).unwrap();

        self.var[x] = new_val.to_string();
        self._chk_mem();

    }
    pub fn del_var(&mut self, name: &str) {
        let x = self.var_name.iter().position(|x| *x == name).unwrap();
        self.var.remove(x);
        self.var_name.remove(x);
    }
    pub fn _chk_mem(&self){
        if self.max_mem  != 0{
            let mem_foot = _mem::get_size(self.var.clone()) + _mem::get_size(self.var_name.clone());
            

            if mem_foot > self.max_mem{
                panic!("You are using to much memory, you are using: {} bytes while you are only allowed to use: {} bytes",mem_foot,self.max_mem);
                /*println!("{},{}",self.var.len(),self.var_name.len());
                println!("{},{}",mem::size_of_val(&self.var),mem::size_of_val(&self.var_name));*/
                

            }
        }

    }
    pub fn set_max_mem(&mut self, max: u64){
        self.max_mem = max;
    }
    pub fn _dump(&mut self) {
        for x in 0..self.var_name.len() {
            print!("{}: ", self.var_name[x]);
            print!("{}\n", self.var[x]);
        }
    }
}
