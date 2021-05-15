//use crate::small;
use crate::lexer;
use crate::lexer::TT;

#[derive(Clone, Debug, PartialEq)]
pub enum Misc {
    Stop,
    IfStop,
    Unknown,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Var {
    Sstring(String),
    Var(String),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Prints(Var),
    If(Var, String, Var),

    Fn(String, [i64; 2], Vec<String>),
    Loop(String, [i64; 2], Vec<String>, Vec<String>, i64),
    Run(String, [i64; 2], Vec<String>, Vec<String>),

    Misc(Misc),
    Get(String),

    Var([String; 2]),
    Change(String, Var),
    Delete(String),
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
// parses the garbage that comes out of the lexer
pub fn parser(code: lexer::Coder) -> Parse {
    let mut parsed = Parse::new();
    #[allow(unused_variables)]
    let curlin = 0;
    let mut modif = 0;
    for line in 0..code.lex.clone().len() {
        let line = line + modif;
        if line >= code.lex.clone().len() {
            break;
        }
        match code.lex[line].clone() {
            TT::LBracket => {
                //let codes = code.lex.clone();
                let code = code.clone();
                // lesson to self never write (self) in a struct its just stupid
                let next_rb = code.next(TT::RBracket, line);
                // calls the sub parser
                let x = sub_parser([line + 1, next_rb], code.clone());
                // push whatever comes out of sub parser to the Parsed struct
                parsed.push(x);
                // find the next line
                modif += next_rb - line;
            }
            a => panic!(format!("MP command not found {:#?}", a)),
        }
        //curlin += 1;
    }
    return parsed;
}
// this thing actually does the most
pub fn sub_parser(pos: [usize; 2], code: lexer::Coder) -> Command {
    let curlin = pos[0];
    //for line in pos[0]..pos[1] {
    #[allow(unused_assignments)]
    let mut command_return: Command = Command::Misc(Misc::Unknown);
    match code.lex[pos[0]].clone() {
        TT::LParen => {
            // LParen is for when the new function comes ()
            let code = code.clone();
            // com and args vectors
            // args is what gets the things after com gets parsed into so like [(com)args]
            let mut com: Vec<String> = Vec::new();
            let mut args: Vec<String> = Vec::new();
            //assembles together the com it only accepts letters and spaces
            for codes in curlin + 1..code.next(TT::RParen, pos[0]) {
                //
                match code.lex[codes].clone() {
                    TT::Letter(a) => {
                        com.push(a);
                    }
                    TT::WhiteSpace => {
                        com.push(" ".to_string());
                    }
                    _ => panic!("sub parsing error "),
                }
            }

            let com = com.join("");
            // then finds the right fit from com. kinda like thoes shitty find love programs
            match com.as_str() {
                "print" => {
                    // this makes the print it first gets and sees if its a variable in the args compartment or a string
                    // it then pushes that to the parsed code
                    let mut is_str = false;
                    let mut ne_q = code.next(TT::RParen, pos[0]) + 1;
                    let mut ne_q2 = code.next(TT::RBracket, pos[0]);
                    for x in code.next(TT::RParen, pos[0]) + 1..code.next(TT::RBracket, pos[0]) {
                        if code.lex[x] == TT::Quotation {
                            is_str = true;
                        }
                    }
                    if is_str {
                        ne_q = code.next(TT::Quotation, pos[0]) + 1;
                        ne_q2 = code.next(TT::Quotation, ne_q);
                    }

                    for codes in ne_q..ne_q2 {
                        //
                        //println!("{},{}",code.next(TT::RParen)+1,pos[1]-1);

                        match code.lex[codes].clone() {
                            TT::Letter(a) => {
                                args.push(a);
                            }
                            TT::Num(a) => {
                                args.push(a);
                            }
                            TT::WhiteSpace => {
                                args.push(" ".to_string());
                            }
                            TT::Char(a) => {
                                args.push(a);
                            }
                            a => panic!(format!("sub parsing error 2: {:#?}", a)),
                        }
                    }
                    let args = args.join("");
                    if is_str {
                        command_return = Command::Prints(Var::Sstring(args));
                    } else {
                        command_return = Command::Prints(Var::Var(args));
                    }
                }
                "var" => {
                    // assembles together the variable
                    // ! rewrite later
                    let ne_q = code.next(TT::RParen, pos[0]) + 1;
                    let ne_q2 = code.next(TT::RBracket, pos[0]);
                    let mut val: Vec<String> = Vec::new();
                    let mut push_to_val: bool = false;

                    for codes in ne_q..ne_q2 {
                        //
                        //println!("{},{}",code.next(TT::RParen)+1,pos[1]-1);

                        match code.lex[codes].clone() {
                            TT::Letter(a) => {
                                if push_to_val {
                                    val.push(a);
                                } else {
                                    args.push(a);
                                }
                            }
                            TT::Num(a) => {
                                if push_to_val {
                                    val.push(a);
                                } else {
                                    args.push(a);
                                }
                            }
                            TT::WhiteSpace => {
                                if push_to_val {
                                    val.push(" ".to_string());
                                } else {
                                }
                                //args.push(" ".to_string());
                            }
                            TT::Char(a) => {
                                args.push(a);
                            }
                            TT::Quotation => {
                                push_to_val = true;
                            }
                            a => panic!(format!("sub parsing error 2: {:#?}", a)),
                        }
                    }

                    let args = args.join("");
                    let val = val.join("");

                    //args.split("");

                    command_return = Command::Var([args, val]);
                }
                "if" => {
                    // parses together the if statement
                    let ne_q = code.next(TT::RParen, pos[0]) + 1;
                    let ne_q2 = code.next(TT::WhiteSpace, ne_q);
                    let x1 = collect_str([ne_q, ne_q2], code.clone());
                    let x1 = parse_str_var(x1);
                    // above: gets the first argument below: gets the if statement like == or !=
                    let ne_q = ne_q2 + 1;
                    let ne_q2 = code.next(TT::WhiteSpace, ne_q);
                    let _xe = collect_str([ne_q, ne_q2], code.clone());
                    //let _xe = parse_str_var(_xe);
                    // below: gets the last argument
                    let ne_q = ne_q2 + 1;
                    let ne_q2 = code.next(TT::RBracket, ne_q);
                    let x2 = collect_str([ne_q, ne_q2], code.clone());
                    let x2 = parse_str_var(x2);
                    //println!("if {:#?} {:#?} {:#?}",x1,_xe,x2);

                    command_return = Command::If(x1, _xe, x2);
                }
                "if stop" => {
                    // just the stop
                    command_return = Command::Misc(Misc::IfStop);
                }
                "edit" => match var_par([pos[0], pos[1]], code) {
                    // finds the edit and if its a variable that wants to be the new value
                    // or just text
                    Command::Change(a, b) => {
                        command_return = Command::Change(a, b);
                    }
                    _ => {
                        panic!("this should never happen");
                    }
                },
                "drop" | "dump" => {
                    let var = collect_str([code.next(TT::RParen, pos[0]) + 1, pos[1]], code);
                    match parse_str_var(var.clone()) {
                        Var::Sstring(_) => {
                            panic!("To dump a variable it needs to be a variable not a string");
                        }
                        _ => {}
                    }
                    command_return = Command::Delete(var);
                }
                a => {
                    //println!();

                    panic!(format!("command not found {}", a))
                }
            }
        }
        a => panic!(format!("SP command not found {:#?} line {}", a, curlin)),
    }
    //curlin += 1;

    //}

    return command_return;
}

fn parse_str_var(str: String) -> Var {
    //parse string and retruns var that contains Sstring and Var
    let mut str = str;
    let mut am_of = 0;
    let mut _am_of_le = 0;

    for x in str.chars() {
        match x {
            '"' => am_of += 1,
            _ => _am_of_le += 1,
        }
    }
    if am_of == 2 {
        str.retain(|x| x != '"');

        Var::Sstring(str)
    } else {
        Var::Var(str)
    }
}
fn collect_str(pos: [usize; 2], code: lexer::Coder) -> String {
    let mut args: Vec<String> = Vec::new();
    // collect a str from random lexer garbage between pos[0] and pos[1]
    for codes in pos[0]..pos[1] {
        //
        //println!("{},{}",code.next(TT::RParen)+1,pos[1]-1);

        match code.lex[codes].clone() {
            TT::Letter(a) => {
                args.push(a);
            }
            TT::Num(a) => {
                args.push(a);
            }
            TT::WhiteSpace => {
                args.push(" ".to_string());
            }
            TT::Char(a) => {
                args.push(a);
            }
            TT::Quotation => {
                args.push(r#"""#.to_string());
            }
            a => panic!(format!("sub_ parsing error 1: {:#?}", a)),
        }
    }
    args.join("")
}

fn var_par(pos: [usize; 2], code: lexer::Coder) -> Command {
    // this does what var up in the sub parser does
    let ne_q = code.next(TT::RParen, pos[0]) + 1;
    let ne_q2 = code.next(TT::RBracket, pos[0]);
    let mut val: Vec<String> = Vec::new();
    let mut args: Vec<String> = Vec::new();

    let mut push_to_val: bool = false;

    for codes in ne_q..ne_q2 {
        //
        //println!("{},{}",code.next(TT::RParen)+1,pos[1]-1);

        match code.lex[codes].clone() {
            TT::Letter(a) => {
                if push_to_val {
                    val.push(a);
                } else {
                    args.push(a);
                }
            }
            TT::Num(a) => {
                if push_to_val {
                    val.push(a);
                } else {
                    args.push(a);
                }
            }
            TT::WhiteSpace => {
                if push_to_val {
                    val.push(" ".to_string());
                } else {
                    push_to_val = true;
                }
                //args.push(" ".to_string());
            }
            TT::Char(a) => {
                args.push(a);
            }
            TT::Quotation => {
                if push_to_val {
                    val.push(r#"""#.to_string());
                } else {
                }
            }
            a => panic!(format!("sub parsing error 2: {:#?}", a)),
        }
    }
    // it just returns Command::Change instead
    let args = args.join("");
    let vals = val.join("");
    let val = parse_str_var(vals.clone());
    //println!("{:#?},{}", val, vals);

    //args.split("");

    return Command::Change(args, val);
}
