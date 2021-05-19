use crate::parser;
use crate::parser::{Command, Parse};
use crate::var::Var;
//what the public sees
pub fn inter(size: [usize; 2], code: Parse) -> Var {
    let mut vars = Var::new();
    inter_back(size, code, &mut vars)
}
// the disgusting backend
pub fn inter_back(size: [usize; 2], code: Parse, vars: &mut Var) -> Var {
    let mut _modif = 0;
    //let mut vars = vars;
    let size_1 = size[0];
    let size_2 = size[1];

    for pos_ in size_1..size_2 {
        let pos_ = pos_ + _modif;
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
                vars.new_var(&a[0], &a[1]);
            }
            // executes the if
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
                    "!=" => {
                        //println!("{},{},",get_data(x1,vars),get_data(x2,vars));

                        if get_data(x1, vars) != get_data(x2, vars) {
                            skip = false;
                        } else {
                        }
                    }

                    a => {
                        panic!("{} can not be used in a if statement ", a)
                    }
                }
                if skip {
                    // skip until the stop command if the if is wrong 
                    let cur = code.parsed_data.clone();
                    // get the stop 
                    let tt = Command::Misc(parser::Misc::IfStop);
                    // find the pos 
                    let x = cur[pos_..size_2].iter().position(|x| *x == tt);
                    let mut _pos = 0;
                    // now its just to jump
                    match x {
                        Some(x) => {
                            _pos = x + pos_;
                            //println!("test {}", pos);
                        }
                        None => {
                            panic!("cannot find the stop for the if")
                        }
                    }
                    //println!("xxxx{},{},{:#?}", pos, pos_, cur[pos]);
                    // dubble check that its correct
                    match cur[_pos] {
                        Command::Misc(parser::Misc::IfStop) => {
                            /*println!("pos= {}",(pos_ - _modif));
                            println!("xxxx{},{},{:#?}", _pos, pos_, cur[_pos]);*/

                            _modif = _pos - pos_;

                            //break;
                        }
                        _ => {}
                    }
                }
            }
            //changes variables
            Command::Change(a, b ,c) => {
                use parser::Math;
                if c != Math::Pass{
                    //println!("{:#?} {:#?}",data,c);
                    // this is for plus minus and stuff like that
                    match c{
                        Math::Minus(ch) =>{
                            let f = vars.get_var(a.clone()).parse::<i64>().unwrap() - ch;
                            vars._up_var(a.as_str(), f.to_string().as_str());
                            
                        }
                        Math::Plus(ch) =>{

                            let f = vars.get_var(a.clone()).parse::<i64>().unwrap() + ch;
                            
                            vars._up_var(a.as_str(), f.to_string().as_str());
                            
                        }
                        Math::Times(ch) =>{
                            let f = vars.get_var(a.clone()).parse::<i64>().unwrap() * ch;
                            vars._up_var(a.as_str(), f.to_string().as_str());
                            
                        }
                        Math::Div(ch) =>{
                            let f = vars.get_var(a.clone()).parse::<i64>().unwrap() / ch;
                            vars._up_var(a.as_str(), f.to_string().as_str());
                            
                        }
                        _=>{
                            panic!("why do i need this? oh right i dont")
                        }
                    }

                }else{
                let data = get_data(b.clone(), vars);

                //println!("chan {}", data);
                vars._up_var(a.as_str(), data.as_str());
                }
                
            }
            Command::Delete(a) => {
                vars.del_var(a.as_str());
            }
            Command::Misc(parser::Misc::IfStop) => {}

            _ => {
                panic!("function not found");
            }
        }
    }
    return vars.clone();
}
// get the data from parser::Var
fn get_data(v: parser::Var, vars: &mut Var) -> String {
    match v {
        parser::Var::Var(a) => return vars.get_var(a),
        parser::Var::Sstring(a) => return a,
    }
}
fn get_str(v: parser::Var) -> String {
    match v {
        parser::Var::Var(a) => return a,
        parser::Var::Sstring(a) => return a,
    }
}
