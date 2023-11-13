use crate::tokens::Token;
use nodes::{AstNode, Expression};
use thiserror::Error;
use crate::lexer::Lexer;
use crate::nodes::{Identifier, MathExpr, MathOp};

mod lexer;
pub mod nodes;
pub mod tokens;

/**
 * Given pattern(s) to match with, returns a Err(ParserErr) if no matches found, if found, returns the result of the expression
 **/
macro_rules! expect {
    ($self:expr, $skip_empty:expr, $($pat:pat => $expr:expr), *$(,)?) => {
        {
            if $skip_empty {
                $self.skip_empty();
            }
            
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

/**
 * Returns true if the next significant token matches any pattern given  
 **/
macro_rules! ignore_empty_match {
    ($self:expr, $($pat:pat), *$(,)?) => {
        {
            let mut current_pos = $self.cursor;
            
            loop {
                if let Some((t, _)) = $self.tokens.get(current_pos) {
                    if t.is_insignificant() { 
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

#[derive(Debug)]
pub struct PropsParser {
    tokens: Vec<(Token, usize)>,
    line: usize,
    cursor: usize,
}

#[allow(dead_code)]
impl PropsParser {
    pub fn new(source: String) -> Self {
        let tokens = Lexer::lex(source);
        println!("{:?}", &tokens);

        PropsParser {
            tokens,
            line: 0,
            cursor: 0,
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

    fn parse_ident(&mut self) -> Result<Identifier, ParserErr> {
        todo!()
    }

    fn parse_math_expr(&mut self) -> Result<MathExpr, ParserErr> {
        todo!()
    }

    pub fn parse_additive_expr(&mut self) -> Result<MathExpr, ParserErr> {
        let mut left = MathExpr::Literal(expect!(self, true, Token::Number(num) => Ok(num))?.clone());

        while ignore_empty_match!(self, Token::Addition, Token::Subtraction) {
            let addition = expect!(self, true, Token::Addition => Ok(true), Token::Subtraction => Ok(false))?;
            let right = expect!(self, true, Token::Number(num) => Ok(num))?;
            left = MathExpr::BinaryOp(
                Box::new(left),
                Box::new(MathExpr::Literal(right.clone())),
                if addition { MathOp::Add } else { MathOp::Sub },
            );
        }

        Ok(left)
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
        if self.cursor >= self.tokens.len() {
            return None;
        }

        let tok = &self.tokens[self.cursor];

        if let Token::Newline = tok.0 {
            self.line += 1;
        }
        
        self.cursor += 1;

        Some(tok)
    }

    fn peek(&mut self) -> Option<&(Token, usize)> {
        let i = self.cursor;

        if i >= self.tokens.len() {
            return None;
        }

        Some(&self.tokens[i])
    }
}