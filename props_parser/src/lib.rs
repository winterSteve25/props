use std::collections::VecDeque;
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
                $(Some(($pat, _)) => $expr,)*
                #[allow(unreachable_patterns)]
                Some((token, pos)) => {
                    Err(ParserErr::UnexpectedToken {
                        line,
                        pos: pos,
                        token: token,
                    })
                }
                None => panic!("This should not happen")
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
            let mut current_pos = 0;
            
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
}

#[derive(Debug)]
pub struct PropsParser {
    tokens: VecDeque<(Token, usize)>,
    line: usize,
}

#[allow(dead_code)]
impl PropsParser {
    pub fn new(source: String) -> Self {
        let tokens = Lexer::lex(source);
        PropsParser {
            tokens: VecDeque::from(tokens),
            line: 0,
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
    
    pub fn parse_math_expr(&mut self) -> Result<MathExpr, ParserErr> {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Result<MathExpr, ParserErr> {
        let mut left = self.parse_multiplicative_expr()?;

        while ignore_empty_match!(self, Token::Addition, Token::Subtraction) {
            let operation = expect!(self, true, Token::Addition => Ok(MathOp::Add), Token::Subtraction => Ok(MathOp::Sub))?;
            let right = self.parse_multiplicative_expr()?;
            left = MathExpr::BinaryOp(
                Box::new(left),
                Box::new(right),
                operation
            );
        }

        Ok(left)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<MathExpr, ParserErr> {
        let mut left = self.parse_parenth_expr()?;

        while ignore_empty_match!(self, Token::Multiplication, Token::Division, Token::Power, Token::Mod) {
            let operation = expect! {
                self,
                true,
                Token::Multiplication => Ok(MathOp::Mul),
                Token::Division => Ok(MathOp::Div),
                Token::Power => Ok(MathOp::Pow),
                Token::Mod => Ok(MathOp::Mod)
            }?;
            
            let right = self.parse_parenth_expr()?;
            
            left = MathExpr::BinaryOp(
                Box::new(left),
                Box::new(right),
                operation
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

    fn next(&mut self) -> Option<(Token, usize)> {
        match self.tokens.pop_front() {
            None => None,
            Some(t) => {
                if let Token::Newline = t.0 {
                    self.line += 1;
                }

                Some(t)
            }
        }
    }

    fn peek(&mut self) -> Option<&(Token, usize)> {
        match self.tokens.get(0) {
            None => None,
            Some(t) => Some(t)
        }
    }
}