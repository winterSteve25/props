use crate::tokens::Token;
use nodes::{AstNode, Expression};
use std::{iter::Peekable, vec::IntoIter};
use thiserror::Error;

pub mod lexer;

mod nodes;
mod tokens;

macro_rules! expect {
    ($self:expr, $($pat:pat => $expr:expr), *$(,)?) => {
        match $self.tokens.next() {
            $(
                Some(($pat, pos)) => {
                    $self.pos = pos;
                    return $expr;
                },
            )*
            #[allow(unreachable_patterns)]
            Some((token, pos)) => {
                $self.pos = pos;

                Err(ParserErr::UnexpectedToken {
                    line: $self.line,
                    pos,
                    token
                })
            },
            None => Err(ParserErr::UnexpectedEOF)
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
pub struct PropsParser {
    tokens: Peekable<IntoIter<(Token, usize)>>,
    line: usize,
    pos: usize,
}

#[allow(dead_code)]
impl PropsParser {
    pub fn new(source: String) -> Self {
        let tokens = lexer::lex(source);
        println!("{:?}", &tokens);

        PropsParser {
            tokens: tokens.into_iter().peekable(),
            line: 0,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut result = Vec::new();

        // skip all starting empty tokens
        self.skip_empty();

        while let Ok(Some(node)) = self.parse_node() {
            result.push(node);
        }

        return result;
    }

    fn parse_node(&mut self) -> Result<Option<AstNode>, ParserErr> {
        match self.tokens.next() {
            Some(token) => match &token.0 {
                Token::Ident(id) => expect! {
                    self,
                    // followed by = is an expression would be an assignment
                    Token::Assignment => {
                        let expr = self.parse_expr()?;
                        Ok(Some(AstNode::Assignment { names: vec![id.clone()], expr }))
                    },
                    // if followed by comma it will be deconstructing
                    Token::Comma => {
                        let mut idents = self.parse_idents_comma_delim();
                        idents.insert(0, id.clone());
                        let expr = self.parse_expr()?;
                        Ok(Some(AstNode::Assignment { names: idents, expr }))
                    },
                    // identifier followed by an expression would be an impure function call
                    _ => {
                        let expr = self.parse_expr()?;
                        Ok(Some(AstNode::ImpFuncCall { name: id.clone(), expr }))
                    }
                },
                _ => Err(ParserErr::UnexpectedToken {
                    line: self.line,
                    pos: token.1,
                    token: token.0.clone(),
                }),
            },
            None => Ok(None),
        }
    }

    fn parse_expr(&mut self) -> Result<Expression, ParserErr> {
        todo!()
    }

    fn parse_idents_comma_delim(&mut self) -> Vec<String> {
        todo!()
    }

    fn skip_empty(&mut self) {
        while let Some(tok) = self.tokens.peek() {
            if let Token::Newline = tok.0 {
                self.line += 1;
            }

            if !tok.0.is_insignificant() {
                break;
            }

            self.tokens.next();
        }
    }
}
