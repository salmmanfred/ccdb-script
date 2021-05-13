
mod lexer;
mod parser;
pub mod exc;

pub fn run(code: String){
    let lex = lexer::lext(code);
    let parsed_code = parser::parser(lex);
    exc::inter([0,parsed_code.parsed_data.len()],parsed_code);

    
}