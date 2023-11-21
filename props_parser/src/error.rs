use colored::Colorize;
use thiserror::Error;
use crate::tokens::Token;

#[derive(Error, Debug)]
pub enum ParserErr {
    #[error("Unexpected token {token:?} at line {line} pos {pos}")]
    UnexpectedToken {
        line: usize,
        pos: usize,
        token: Token,
    },
}

impl ParserErr {
    pub fn print(&self, source: &Vec<String>) {
        println!();
        println!("Parsing Error: {}", self.to_string().red());

        let (line, pos, token) = match self {
            ParserErr::UnexpectedToken { line, pos, token } => (line, pos, token),
        };

        let text = &source[*line - 1];
        println!("  | ");
        println!("{} | {}", line.to_string().blue(), text);

        let pointer: String = " ".repeat(if pos > &0 { pos - token.len() + 1 } else { 0 } ) + &"^".repeat(token.len());
        println!("  | {}", pointer.red());
        println!();
    }
}
