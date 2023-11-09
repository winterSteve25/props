use nodes::Statement;
use crate::tokens::Token;

pub mod lexer;

mod nodes;
mod tokens;

pub fn parse(source: String) -> Vec<Statement> {
    let tokens = lexer::lex(source);
    let mut statements = vec![];
    let mut i = 0;

    println!("{:?}", tokens);
    statements
}

fn next(i: usize) -> Option<Token> {

}
