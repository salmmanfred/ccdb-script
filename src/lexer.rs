#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TT {
    Let,
    WhiteSpace,
    NewLine,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Plus,
    PlusAssign,
    Minus,
    MinusAssign,
    Slash,
    Star,
    Percent,
    Bang,
    BangEqual,
    Equal,
    Assign,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    And,
    Or,
    Struct,
    If,
    Else,
    Elif,
    True,
    False,
    Function,
    For,
    While,
    Print,
    EOF,
    Hash,
    Semicolon,
    Break,
    LBracket,
    RBracket,
    Letter(String),
    Num(String),
    Char(String),
    Quotation,
    Unknown,
}
#[derive(Clone)]
pub struct Coder {
    pub lex: Vec<TT>,
}
impl Coder {
    pub fn push(&mut self, tt: TT) {
        self.lex.push(tt);
    }
    pub fn new() -> Coder {
        Coder { lex: Vec::new() }
    }
    pub fn next(&self, _tt: TT, current_line: usize) -> usize {
        for x in current_line..self.lex.len() {
            //println!("{:#?},{}",self.lex[x],x);

            if self.lex[x] == _tt {
                /*println!("{}",x);
                println!("{:#?}",_tt);
                println!("{:#?}",self.lex[x]);*/

                return x;
            }
        }
        panic!("Next not found {:#?},{}", _tt, current_line)
    }
}

pub fn lext(code: String) -> Coder {
    let code = code;
    let code: Vec<String> = code.chars().map(|x| x.to_string()).collect();
    let code: Vec<&str> = code.iter().map(|x| x.as_str()).collect();
    let mut holder = Coder::new();
    for char in code.into_iter() {
        match char {
            "[" => holder.push(TT::LBracket),
            "]" => holder.push(TT::RBracket),
            "(" => holder.push(TT::LParen),
            ")" => holder.push(TT::RParen),
            " " => holder.push(TT::WhiteSpace),
            //"=" => holder.push(TT::Equal),
            r#"""# => holder.push(TT::Quotation),

            "\n" => {}
            a => {
                let char: Vec<char> = a.chars().collect();
                match char[0] {
                    'a'..='z' => holder.push(TT::Letter(a.to_string())),
                    '0'..='9' => holder.push(TT::Num(a.to_string())),
                    a => holder.push(TT::Char(a.to_string())),
                    // _=> panic!("Thats not a letter")
                }
            } //_=> holder.push(TT::Unknown)
        }
    }
    return holder;
}
