pub mod exc;
mod lexer;
mod parser;
mod var;

pub fn run_parsed(code: parser::Parse) {
    exc::inter([0, code.parsed_data.len()], code);
}
pub fn parse(code: String) -> parser::Parse {
    let lex = lexer::lext(code);
    let parsed_code = parser::parser(lex);
    parsed_code
}

pub fn run(code: String) {
    let lex = lexer::lext(code);
    let parsed_code = parser::parser(lex);
    //drop(lex);
    exc::inter([0, parsed_code.parsed_data.len()], parsed_code);
}
