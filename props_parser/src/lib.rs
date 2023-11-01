use thiserror::Error;
use tokens::Token;
use nodes::Statement;

mod nodes;
mod tokens;

#[derive(Debug, Error)]
pub enum ParseErr {
    #[error("Expected token at {line:?}:{column:?}")]
    ExpectedToken {
        line: usize,
        column: usize,
        token: Token
    },
    #[error("Un")]
    UnexpectedToken{
        line: usize,
        column: usize,
        token: Token
    }
}

pub fn parse(source: String) -> Result<Vec<Statement>, ()> {
    
    let chars = source.chars();
    let statements = Vec::<Statement>::new();
    let errs = Vec::<ParseErr>::new();

    let mut line_num = 0;
    let mut char_num = 0;

    let mut i = 0;

    loop {
    }

    Ok(statements)
}
