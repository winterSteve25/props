use std::collections::VecDeque;
use crate::tokens::Token;
use nodes::{AstNode, Expression};
use thiserror::Error;
use crate::lexer::Lexer;
use crate::nodes::{Identifier, MathExpr, MathOp};

mod lexer;
mod tokens;
pub mod nodes;
pub mod number;

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
macro_rules! peek_match_ignore_ws {
    ($self:expr, $skip:expr, $($pat:pat), *$(,)?) => {
        {
            let mut current_pos = $skip;
            
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
                Err(err) => panic!("{:?}", err),
            }
        }

        return result;
    }

    fn parse_node(&mut self) -> Result<Option<AstNode>, ParserErr> {
        if let Some((Token::EOF, _)) = self.peek() {
            return Ok(None);
        }

        let ident = self.parse_ident()?;

        // regular assignment
        if peek_match_ignore_ws!(self, 0, Token::Assignment) {
            expect!(self, true, Token::Assignment => Ok(()))?;
            let expr = self.parse_expr()?;
            return Ok(Some(AstNode::Assignment {
                name: ident,
                expr,
            }));
        }

        // impure function call
        Ok(Some(AstNode::ImpFuncCall {
            name: ident,
            arguments: self.parse_ws_delimited_exprs()?,
        }))
    }

    fn parse_expr(&mut self) -> Result<Expression, ParserErr> {
        if peek_match_ignore_ws!(self, 0, Token::Ident(_)) {
            return Ok(Expression::Identifier(self.parse_ident()?));
        }

        // (ident ... -> function call
        if peek_match_ignore_ws!(self, 0, Token::ParenthOpen) && peek_match_ignore_ws!(self, 1, Token::Ident(_)) {
            expect!(self, true, Token::ParenthOpen => Ok(()))?;
            let ident = self.parse_ident()?;
            let arguments = self.parse_ws_delimited_exprs()?;
            expect!(self, true, Token::ParenthClose => Ok(()))?;
            return Ok(Expression::FuncCall {
                func_name: ident,
                arguments,
            });
        }
        
        if peek_match_ignore_ws!(self, 0, Token::Number(_), Token::Subtraction, Token::ParenthOpen) {
            return Ok(Expression::MathExpr(self.parse_math_expr()?));
        }
        
        if peek_match_ignore_ws!(self, 0, Token::StringLiteral(_)) { 
            return expect!(self, true, Token::StringLiteral(str) => Ok(Expression::StrLiteral(str)));
        } 
        
        // if peek_match_ignore_ws!(self, 0, Token::Pipe, Token::FuncOpen) { 
        //     let has_params = expect!(self, true, Token::Pipe => Ok(true), Token::FuncOpen => Ok(false))?;
        //     let mut params = vec![];
        //     
        //     if has_params {
        //         
        //         while !peek_match_ignore_ws!(self, 0, Token::Pipe) {
        //             
        //         }
        //     }
        //     
        //     return Ok(Expression::FuncLiteral {
        //         params: params,
        //         statements: vec![],
        //     })
        // }

        // wouldn't panic because if we reached EOF parse_expr wouldn't be called
        let token = self.next().unwrap();

        Err(ParserErr::UnexpectedToken {
            line: self.line,
            pos: token.1,
            token: token.0,
        })
    }

    fn parse_ws_delimited_exprs(&mut self) -> Result<Vec<Expression>, ParserErr> {
        if let Some((Token::Whitespace, _)) = self.peek() {
            expect!(self, false, Token::Whitespace => Ok(()))?;
            let mut exprs: Vec<Expression> = Vec::new();

            while let Ok(expr) = self.parse_expr() {
                exprs.push(expr);
                if let Some((Token::Whitespace, _)) = self.peek() {
                    expect!(self, false, Token::Whitespace => Ok(()))?;
                } else {
                    break;
                }
            }

            Ok(exprs)
        } else { 
            Ok(vec![])
        }
    }

    pub fn parse_ident(&mut self) -> Result<Identifier, ParserErr> {
        let mut ident = expect!(self, true, Token::Ident(str) => Ok(Identifier::Identifier(str)))?;

        while peek_match_ignore_ws!(self, 0, Token::Period, Token::Comma) {
            let accessor = expect!(self, true, Token::Period => Ok(true), Token::Comma => Ok(false))?;
            let extra_ident = expect!(self, true, Token::Ident(str) => Ok(Identifier::Identifier(str)))?;

            if accessor {
                ident = Identifier::Accessor(Box::new(ident), Box::new(extra_ident));
            } else {
                ident = ident.compound(extra_ident);
            }
        }

        Ok(ident)
    }

    pub fn parse_math_expr(&mut self) -> Result<MathExpr, ParserErr> {
        let mut left = self.parse_multiplicative_expr()?;

        while peek_match_ignore_ws!(self, 0, Token::Addition, Token::Subtraction) {
            let operation = expect!(self, true, Token::Addition => Ok(MathOp::Add), Token::Subtraction => Ok(MathOp::Sub))?;
            let right = self.parse_multiplicative_expr()?;
            left = MathExpr::BinaryOp(
                Box::new(left),
                Box::new(right),
                operation,
            );
        }

        Ok(left)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<MathExpr, ParserErr> {
        let mut left = self.parse_parenth_expr()?;

        while peek_match_ignore_ws!(self, 0, Token::Multiplication, Token::Division, Token::Power, Token::Mod) {
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
                operation,
            );
        }

        Ok(left)
    }

    fn parse_parenth_expr(&mut self) -> Result<MathExpr, ParserErr> {
        if peek_match_ignore_ws!(self, 0, Token::ParenthOpen) {
            expect!(self, true, Token::ParenthOpen => Ok(()))?;
            let result = self.parse_math_expr()?;
            expect!(self, true, Token::ParenthClose => Ok(()))?;
            return Ok(result);
        }

        self.parse_unary_expr()
    }

    fn parse_unary_expr(&mut self) -> Result<MathExpr, ParserErr> {
        if peek_match_ignore_ws!(self, 0, Token::Subtraction) {
            expect!(self, true, Token::Subtraction => Ok(()))?;
            Ok(MathExpr::Negate(Box::new(self.parse_parenth_expr()?)))
        } else {
            expect!(self, true, Token::Number(num) => Ok(MathExpr::Literal(num)))
        }
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