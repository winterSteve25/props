#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Unknown(String),

    Pipe,
    Whitespace,
    FuncOpen,
    FuncClose,
    Assignment,
    
    Plus,
    Minus,
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

    Newline,
    Indent(usize),
    EOF,
}
