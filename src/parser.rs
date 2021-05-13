//use crate::small;
use crate::lexer;
use crate::lexer::TT;


#[derive(Clone)]
pub enum Misc {
    Stop,
    Unknown,
}
#[derive(Clone)]
pub enum Command {
    Prints(String),
    If([i64; 2], [String; 3]),

    Fn(String, [i64; 2], Vec<String>),
    Loop(String, [i64; 2], Vec<String>, Vec<String>, i64),
    Run(String, [i64; 2], Vec<String>, Vec<String>),

    Misc(Misc),
    Get(String),

    MkvI([String; 2]),
    MkvS([String; 2]),
    Change([String; 2]),
    Delete(String),

    MakeFile(String, String),
    MakeFolder(String),
}
#[derive(Clone)]
pub struct Parse {
    pub parsed_data: Vec<Command>,
}
impl Parse {
    pub fn new() -> Parse {
        Parse {
            parsed_data: Vec::new(),
        }
    }
    pub fn push(&mut self, ty: Command) {
        self.parsed_data.push(ty);
    }
    pub fn find_fn(&mut self, name: &str) -> Command {
        for x in 0..self.parsed_data.len() {
            match self.parsed_data[x].clone() {
                Command::Fn(a, _b, _c) => {
                    if a == name {
                        return self.parsed_data[x].clone();
                    }
                }
                _ => {}
            }
        }
        panic!("no function")
    }
}

pub fn parser(code: lexer::Coder) -> Parse {
    let mut parsed = Parse::new();
    let mut curlin = 0;
    let mut modif = 0;
    for line in 0..code.lex.clone().len() {
        
        let line = line + modif;
        if line >= code.lex.clone().len(){
            break;
        }
        match code.lex[line].clone(){
            TT::LBracket =>{
                let codes = code.lex.clone();
                let code = code.clone();
                // lesson to self never write (self) in a struct its just stupid
                let next_rb =  code.next(TT::RBracket,line);
                let x = sub_parser([line+1,next_rb],code.clone());
                parsed.push(x);
                modif += next_rb - line ;
            }
            a => panic!(format!("MP command not found {:#?}",a))

        }
        //curlin += 1;
    }
    return parsed;
}
pub fn sub_parser(pos:[usize;2],code: lexer::Coder)->Command{
    let mut curlin = pos[0];
    //for line in pos[0]..pos[1] {
        let mut command_return: Command = Command::Misc(Misc::Unknown);
        match code.lex[pos[0]].clone(){
            TT::LParen =>{
                let code = code.clone();
                let mut com: Vec<String> = Vec::new();
                let mut args: Vec<String> = Vec::new();

                
                for codes in curlin+1..code.next(TT::RParen,pos[0]){
                    //
                    match code.lex[codes].clone(){
                        TT::Letter (a) =>{
                            com.push(a);
                        }
                        TT::WhiteSpace =>{
                            com.push(" ".to_string());
                        }
                        _=> panic!("sub parsing error ")
                    }
                }
                
                let com = com.join("");

              
                match com.as_str(){
                    "print" => {
                        for codes in code.next(TT::RParen,pos[0])+1..pos[1]{
                            //
                            //println!("{},{}",code.next(TT::RParen)+1,pos[1]-1);
        
                            match code.lex[codes].clone(){
                                TT::Letter (a) =>{
                                    args.push(a);
                                }
                                TT::Num (a) =>{
                                    args.push(a);
                                }
                                TT::WhiteSpace =>{
                                    args.push(" ".to_string());
                                }
                                a=> panic!(format!("sub parsing error 2: {:#?}",a))
                            }
                        }
                        let args = args.join("");

                        command_return = Command::Prints(args);
                    }
                    a=> {
                        //println!();
                        
                        panic!(format!("command not found {}",a))
                    },
                }
                

            }
            a => panic!(format!("SP command not found {:#?} line {}",a,curlin))

        }
        curlin += 1;
      
    //}

    return command_return;
}
