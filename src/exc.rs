use crate::parser;
use crate::parser::{Command, Parse};
use crate::var::Var;
use crate::custom::Custom;
//what the public sees
pub fn inter(size: [usize; 2], code: Parse) -> Var {
    let mut vars = Var::new();
    let cus = Custom::new();
    inter_back(size, code, &mut vars, cus)
}
// the disgusting backend
pub fn inter_back(size: [usize; 2], code: Parse, vars: &mut Var,custom:Custom ) -> Var {
    let mut _modif = 0;
    //let mut vars = vars;
    let size_1 = size[0];
    let size_2 = size[1];

    for pos_ in size_1..size_2 {
        //print!("pos_{}",pos_);
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
               // println!("pos{},f {}",pos_,skip);
                if skip {
                    // skip until the stop command if the if is wrong 
                    let cur = code.parsed_data.clone();
                    // get the stop 
                    //let tt = Command::Misc(parser::Misc::IfStop);
                    // find the pos 
                    //let x = cur[pos_..size_2].iter().position(|x| *x == tt);
                    let mut inc = 1; // this must be zero for it to have found the correct stop

                    let mut correct_stop = 0;// the correct if stop
                    let mut post = pos_; // current position of the pointer
                    for x in cur[pos_+1..size_2].iter(){//lets now find that stop if 
                        match x {
                            Command::If (_,_,_)=>{// adds to the inc if there are more ifs in there
                                inc += 1;
                            }
                            Command::Misc(parser::Misc::IfStop) =>{// if there is a if stop it removes a inc
                                inc -= 1;

                                if inc <= 0{
                                    correct_stop = post;
                                    //println!("{},{}",correct_stop,inc);
                                    break;
                                }

                            }
                            _=>{}
                        }
                        post += 1;

                    }
                    let mut _pos = 0;
                    // now its just to jump
                    /*match  {
                        Some(x) => {
                            _pos = x + pos_;
                            //println!("test {}", pos);
                        }
                        None => {
                            panic!("cannot find the stop for the if")
                        }
                    }*/
                    _pos = correct_stop ;
                    _modif = _pos - pos_;
                   // println!("{},{}",_modif,(_modif + pos_));

                    
                    //println!("xxxx{},{},{:#?}", pos, pos_, cur[pos]);
                    // dubble check that its correct
                    
                    if _pos >= size_2{
                        break;
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
            Command::Cus(a,b) => {
                let mut args: Vec<String> = Vec::new();
                
                if custom.is_no_in(a.clone()){
                    
                }else{
                    for x in b{
                        args.push(get_data(x,vars));
                    }
                }
                match custom.run_fn(a, args,vars){
                    Some (a) =>{
                        vars.new_var( format!("R{}",a).as_str(),a.as_str());

                    }
                    None =>{
                    
                    }
                }
            }
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
fn _get_str(v: parser::Var) -> String {
    match v {
        parser::Var::Var(a) => return a,
        parser::Var::Sstring(a) => return a,
    }
}
