use crate::types::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    StringLiteral(String),
    Unknown(String),
    Number(Number),

    Pipe,
    TypeAnnotator,
    Comma,
    Period,
    FuncOpen,
    FuncClose,
    Assignment,
    Return,

    Addition,
    Subtraction,
    Multiplication,
    Division,
    Mod,
    Power,

    ParenthOpen,
    ParenthClose,
    
    Not,
    Equality,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,

    Whitespace,
    Newline,
    Indent(usize),
    EOF,
}

impl Token {
    pub fn is_insignificant(&self) -> bool {
        match self {
            Token::Whitespace => true,
            Token::Newline => true,
            Token::Indent(_) => true,
            _ => false
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false,
        }
    }
    
    pub fn len(&self) -> usize {
        match self {
            Token::Ident(id) => id.len(),
            Token::Unknown(i) => i.len(),
            Token::Number(num) => num.len(),
            Token::Equality => 2,
            Token::Indent(lvl) => lvl * 4,
            Token::EOF => 0,
            _ => 1,
        }
    } 
}
