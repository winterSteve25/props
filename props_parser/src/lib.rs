use nodes::{Statement, Expression};
use crate::tokens::Token;

pub mod lexer;

mod nodes;
mod tokens;

#[allow(dead_code)]
pub struct Parser {
    i: usize,
    tokens: Vec<(Token, usize)>,
    line: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(source: String) -> Self {
        let tokens = lexer::lex(source);
        println!("{:?}", tokens);
        
        Parser { 
            i: 0, 
            tokens,
            line: 0,
        }
    }

    fn peak(&self) -> Option<&(Token, usize)> {
        if self.i + 1 >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[self.i + 1])
    }

    fn next(&mut self) -> Option<&(Token, usize)> {
        if self.i >= self.tokens.len() {
            return None;
        }

        let result = Some(&self.tokens[self.i]);
        self.i += 1;
        result
    }

    fn skip_ws(&mut self) {
        while let Some((Token::Whitespace, _)) = self.peak() {
            self.i += 1;
        }
    }
}
