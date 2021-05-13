use crate::parser;
use crate::parser::{Command, Parse};

pub fn inter(size: [usize; 2], code: Parse) {
    //let mut vars = Var::new();
    inter_back(size, code/*, &mut vars*/)
}
pub fn inter_back(size: [usize; 2], code: Parse/* , vars: &mut Var*/) {
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
            Command::Prints(a) => {
                println!("{}", a);
            }
            
            _ => {
                panic!("function not found");
            }
        }
    }
}