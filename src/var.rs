#[derive(Clone)]
pub struct Var {
    pub var: Vec<String>,
    pub var_name: Vec<String>,
}
impl Var {
    pub fn new() -> Var {
        Var {
            var: Vec::new(),
            var_name: Vec::new(),
        }
    }
    //make a new Var thats a string the only 2 variables in this language
    pub fn new_var(&mut self, name: &str, value: &str) {
        self.var.push(value.to_string());
        self.var_name.push(name.to_string());
    }
    // get a Var from memory #MEM:var
    pub fn get_var(&mut self, name: String) -> String {
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
    }
    pub fn del_var(&mut self, name: &str) {
        let x = self.var_name.iter().position(|x| *x == name).unwrap();
        self.var.remove(x);
        self.var_name.remove(x);
    }
    pub fn _dump(&mut self) {
        for x in 0..self.var_name.len() {
            print!("{}: ", self.var_name[x]);
            print!("{}\n", self.var[x]);
        }
    }
}
