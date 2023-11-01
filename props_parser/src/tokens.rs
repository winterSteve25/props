#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    KeyW(Keyword),
    Pipe,
    Whitespace,
    FuncOpen,
    FuncClose,
    Unexpected(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Impure,
}
