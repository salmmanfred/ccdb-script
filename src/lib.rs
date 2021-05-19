pub mod exc;
mod lexer;
mod parser;
pub mod var;
//use std::time::Instant;
pub fn run_parsed(code: parser::Parse) -> var::Var {
    exc::inter([0, code.parsed_data.len()], code)
}
pub fn parse(code: String) -> parser::Parse {
    let lex = lexer::lext(code);
    let parsed_code = parser::parser(lex);
    parsed_code
}

pub fn run(code: String) -> var::Var {
    let lex = lexer::lext(code);
    let parsed_code = parser::parser(lex);
    //drop(lex);
    //let now = Instant::now();
    let x = exc::inter([0, parsed_code.parsed_data.len()], parsed_code);
    //println!("Extime: {}", now.elapsed().as_millis());
    x
}

pub fn run_parsed_var(code: parser::Parse, vars: &mut var::Var) -> var::Var{
    exc::inter_back([0, code.parsed_data.len()], code, vars)
}