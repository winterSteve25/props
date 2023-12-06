use colored::Colorize;
use log::error;
use thiserror::Error;
use crate::tokens::Token;
use crate::types::Type;
use crate::util::Access;

#[derive(Error, Debug)]
pub enum ParserErr {
    #[error("Unexpected token {token:?} at line {line} pos {pos}")]
    UnexpectedToken {
        line: usize,
        pos: usize,
        token: Token,
    },
    #[error("Can not assign type {type_1} to an identifier of type {type_2}")]
    UnmatchedTypes {
        type_1: Access<Type>,
        type_2: Access<Type>,
    }
}

impl ParserErr {
    pub fn print(&self, source: &Vec<String>) {
        println!();
        println!("Parsing Error: {}", self.to_string().red());

        let (line, pos, token) = match self {
            ParserErr::UnexpectedToken { line, pos, token } => (line, pos, token),
            _ => (&0, &0, &Token::Newline)
        };

        let text = &source[*line - 1];
        println!("  | ");
        println!("{} | {}", line.to_string().blue(), text);

        let pointer: String = " ".repeat(if pos > &0 { pos - token.len() + 1 } else { 0 } ) + &"^".repeat(token.len());
        println!("  | {}", pointer.red());
        println!();
    }
}
