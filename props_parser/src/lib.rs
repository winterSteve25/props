use crate::tokens::Token;
use nodes::{AstNode, Expression};
use thiserror::Error;
use crate::lexer::Lexer;
use crate::nodes::Identifier;

pub mod lexer;

mod nodes;
mod tokens;

macro_rules! expect {
    ($self:expr, $($pat:pat => $expr:expr), *$(,)?) => {
        {
            let line = $self.line;
            
            match $self.next() {
                $(
                    Some(($pat, _)) => $expr,
                )*
                #[allow(unreachable_patterns)]
                Some((token, pos)) => {
                    Err(ParserErr::UnexpectedToken {
                        line,
                        pos: pos.clone(),
                        token: token.clone(),
                    })
                }
                None => Err(ParserErr::UnexpectedEOF)
            }
        }
    };
}

macro_rules! ignore_ws_has_token_next {
    ($self:expr, $($pat:pat), *$(,)?) => {
        {
            let mut current_pos = $self.pos;
            
            loop {
                if let Some((t, _)) = $self.tokens.get(current_pos) {
                    if let Token::Whitespace = t { 
                        current_pos += 1;
                    } else { 
                        break;
                    }
                } else { 
                    break;
                }
            }
            
            match $self.tokens.get(current_pos) {
                $(Some(($pat, _)) => true,)*
                _ => false,
            }
        }
    };
}

#[derive(Error, Debug)]
pub enum ParserErr {
    #[error("Unexpected token {token:?} at line {line} pos {pos}")]
    UnexpectedToken {
        line: usize,
        pos: usize,
        token: Token,
    },

    #[error("Expected token {token:?} at line {line} pos {pos}, but {found:?} was found")]
    ExpectedToken {
        line: usize,
        pos: usize,
        token: Token,
        found: Token,
    },

    #[error("Unexpected End of File")]
    UnexpectedEOF,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PropsParser {
    tokens: Vec<(Token, usize)>,
    line: usize,
    pos: usize,
}

#[allow(dead_code)]
impl PropsParser {
    pub fn new(source: String) -> Self {
        let tokens = Lexer::lex(source);
        println!("{:?}", &tokens);

        PropsParser {
            tokens,
            line: 0,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut result = Vec::new();

        // skip all starting empty tokens
        self.skip_empty();

        loop {
            match self.parse_node() {
                Ok(Some(node)) => result.push(node),
                Ok(None) => break,
                Err(err) => println!("{:?}", err),
            }
        }

        return result;
    }

    fn parse_node(&mut self) -> Result<Option<AstNode>, ParserErr> {
        todo!()
    }

    fn parse_expr(&mut self) -> Result<Expression, ParserErr> {
        todo!()
    }
    
    fn parse_ident() -> Result<Identifier, ParserErr> {
        todo!()
    }

    fn skip_empty(&mut self) {
        while let Some(tok) = self.peek() {
            if !tok.0.is_insignificant() {
                break;
            }

            self.next();
        }
    }
    
    fn next(&mut self) -> Option<&(Token, usize)> {
        self.pos += 1;
        
        if self.pos >= self.tokens.len() { 
            return None;
        } 
        
        let tok = &self.tokens[self.pos];
        
        if let Token::Newline = tok.0 { 
            self.line += 1;
        } 
        
        Some(tok)
    }
    
    fn peek(&mut self) -> Option<&(Token, usize)> {
        let i = self.pos + 1;
        
        if i >= self.tokens.len() { 
            return None;
        }
        
        Some(&self.tokens[i])
    }
}
