use crate::parser;
use crate::parser::{Command, Parse};
use crate::var::Var;

pub fn inter(size: [usize; 2], code: Parse) {
    let mut vars = Var::new();
    inter_back(size, code, &mut vars)
}
pub fn inter_back(size: [usize; 2], code: Parse, vars: &mut Var) {
    let mut modif = 0;
    //let mut vars = vars;
    let size_1 = size[0];
    let size_2 = size[1];

    for pos_ in size_1..size_2 {
        let pos_ = pos_ + modif;
        if pos_ >= size_2 {
            break;
        }
        let cur = code.parsed_data[pos_].clone();
        match cur {
            Command::Prints(a) => match a {
                parser::Var::Var(a) => {
                    println!("{}", vars.get_var(a));
                }
                parser::Var::Sstring(a) => {
                    println!("{}", a);
                }
            },
            Command::Var(a) => {
                //println!("new var{},{}", a[0], a[1]);
                vars.new_var_string(&a[0], &a[1]);
            }
            Command::If(x1, e, x2) => {
                let mut skip = true;
                match e.as_str() {
                    "==" => {
                        //println!("{},{},",get_data(x1,vars),get_data(x2,vars));

                        if get_data(x1, vars) == get_data(x2, vars) {
                            skip = false;
                        } else {
                        }
                    }

                    a => {
                        panic!("{} can not be used in a if statement ", a)
                    }
                }
                if skip {
                    let cur = code.parsed_data.clone();

                    for x in pos_..size_2 {
                        match cur[x] {
                            Command::Misc(parser::Misc::IfStop) => {
                                modif = x - pos_;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Command::Change (a,b) =>{
                let data = get_data(b,vars);
                println!("chan {}",data);
                vars._up_var(a.as_str(),data.as_str())
            }
            Command::Misc(parser::Misc::IfStop) => {}

            _ => {
                panic!("function not found");
            }
        }
    }
}

fn get_data(v: parser::Var, vars: &mut Var) -> String {
    match v {
        parser::Var::Var(a) => return vars.get_var(a),
        parser::Var::Sstring(a) => return a,
    }
}
