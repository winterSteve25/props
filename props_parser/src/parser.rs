use std::collections::VecDeque;
use std::rc::Rc;
use crate::error::ParserErr;
use crate::tokens::Token;
use crate::lexer::Lexer;
use crate::nodes::{AstNode, Expression, Identifier, MathExpr, MathOp};
use crate::types::Type;

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
                        line: line + 1,
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

#[derive(Debug)]
pub struct PropsParser {
    tokens: VecDeque<(Token, usize)>,
    source: Vec<String>,
    line: usize,
    parsing_ws_delim: bool,
    ws_delim_in_parenth: bool,
}

#[allow(dead_code)]
impl PropsParser {
    pub fn new() -> Self {
        PropsParser {
            tokens: VecDeque::new(),
            source: vec![],
            line: 0,
            parsing_ws_delim: false,
            ws_delim_in_parenth: false,
        }
    }
    
    pub fn init(&mut self, source: String) {
        let tokens = Lexer::lex(&source);
        self.tokens = VecDeque::from(tokens);
        self.source = source.lines().map(String::from).collect();
    }

    pub fn parse(&mut self) -> (Vec<AstNode>, Vec<ParserErr>) {
        let mut result = Vec::new();
        let mut errs = Vec::new();

        loop {
            // skip all starting empty tokens
            self.skip_empty();

            match self.parse_node() {
                Ok(Some(node)) => result.push(node),
                Ok(None) => break,
                Err(err) => {
                    err.print(&self.source);
                    errs.push(err);
                }
            }
        }

        return (result, errs);
    }

    fn parse_node(&mut self) -> Result<Option<AstNode>, ParserErr> {
        if let None = self.peek() {
            return Ok(None);
        }

        if peek_match_ignore_ws!(self, 0, Token::Return) {
            self.skip_empty();
            self.next();
            let expr = self.parse_expr()?;
            return Ok(Some(AstNode::Return(expr)));
        }

        let ident = self.parse_ident()?;

        // regular assignment
        if peek_match_ignore_ws!(self, 0, Token::Assignment) {
            self.skip_empty();
            self.next();
            let expr = self.parse_expr()?;
            return Ok(Some(AstNode::Assignment(ident, expr)));
        }

        // impure function call
        Ok(Some(AstNode::ImpFuncCall(ident, self.parse_ws_delimited_exprs()?)))
    }

    fn parse_expr(&mut self) -> Result<Expression, ParserErr> {
        let mut expr = if peek_match_ignore_ws!(self, 0, Token::StringLiteral(_)) {
            expect!(self, true, Token::StringLiteral(str) => Ok(Expression::StrLiteral(str)))
        } else if peek_match_ignore_ws!(self, 0, Token::Pipe, Token::FuncOpen) {
            let has_params = expect!(self, true, Token::Pipe => Ok(true), Token::FuncOpen => Ok(false))?;
            let mut params = vec![];

            let statements = if has_params {
                loop {
                    let id = expect!(self, true, Token::Ident(id) => Ok(id))?;
                    let type_ = if peek_match_ignore_ws!(self, 0, Token::TypeAnnotator) {
                        self.skip_empty();
                        self.next();
                        self.parse_type()?
                    } else {
                        Type::Undefined
                    };

                    params.push((id, type_));

                    if let Some((Token::Whitespace, _)) = self.peek() {
                        self.next();
                    }

                    if peek_match_ignore_ws!(self, 0, Token::Pipe) {
                        self.skip_empty();
                        self.next();
                        break;
                    }
                }
                
                if peek_match_ignore_ws!(self, 0, Token::FuncOpen) {
                    self.parse_function_body()?
                } else { 
                    vec![AstNode::Return(self.parse_expr()?)]
                }
            } else {
                self.parse_function_body()?
            };

            Ok(Expression::FuncLiteral {
                params,
                statements,
                return_type: Type::Undefined,
            })
        } else {
            Ok(Expression::MathExpr(self.parse_math_expr()?))
        }?;

        while peek_match_ignore_ws!(self, 0, Token::Comma) {
            self.skip_empty();
            self.next();
            let expr2 = self.parse_expr()?;
            expr = expr.compound(expr2);
        }

        Ok(expr)
    }
    
    fn parse_function_body(&mut self) -> Result<Vec<AstNode>, ParserErr> {
        let mut result = vec![];

        expect!(self, true, Token::FuncOpen => Ok(()))?;

        while !peek_match_ignore_ws!(self, 0, Token::FuncClose) {
            if let Some(ast) = self.parse_node()? {
                result.push(ast);
            } else {
                break;
            }
        }

        expect!(self, true, Token::FuncClose => Ok(()))?;

        Ok(result)
    }

    fn parse_ws_delimited_exprs(&mut self) -> Result<Vec<Expression>, ParserErr> {
        if let Some((Token::Whitespace, _)) = self.peek() {
            self.next();
            
            self.parsing_ws_delim = true;
            let mut exprs: Vec<Expression> = Vec::new();

            while let Ok(expr) = self.parse_expr() {
                exprs.push(expr);
                if let Some((Token::Whitespace, _)) = self.peek() {
                    expect!(self, false, Token::Whitespace => Ok(()))?;
                } else {
                    break;
                }
            }

            self.parsing_ws_delim = false;
            Ok(exprs)
        } else {
            Ok(vec![])
        }
    }
    
    fn parse_type(&mut self) -> Result<Type, ParserErr> {
        if peek_match_ignore_ws!(self, 0, Token::ParenthOpen) {
            self.skip_empty();
            self.next();
            let mut types = vec![Type::Defined(expect!(self, true, Token::Ident(str) => Ok(str))?)];
            
            while peek_match_ignore_ws!(self, 0, Token::Comma) {
                self.skip_empty();
                self.next();
                types.push(Type::Defined(expect!(self, true, Token::Ident(str) => Ok(str))?));
            }
            
            expect!(self, true, Token::ParenthClose => Ok(()))?;
            return Ok(Type::Compound(types));
        }

        let type_ = expect!(self, true, Token::Ident(str) => Ok(str))?;
        Ok(Type::Defined(type_))
    }

    pub fn parse_ident(&mut self) -> Result<Identifier, ParserErr> {
        let mut ident = self.parse_simple_ident()?;

        while peek_match_ignore_ws!(self, 0, Token::Comma) {
            self.skip_empty();
            self.next();
            let rhs = self.parse_simple_ident()?;
            ident = ident.compound(rhs);
        }

        Ok(ident)
    }

    fn parse_simple_ident(&mut self) -> Result<Identifier, ParserErr> {
        let str = expect!(self, true, Token::Ident(str) => Ok(str))?;

        if peek_match_ignore_ws!(self, 0, Token::TypeAnnotator) {
            self.skip_empty();
            self.next();
            let type_ = self.parse_type()?;
            return Ok(Identifier::Identifier(Rc::new(str), Rc::new(type_)));
        }

        let mut ident = Identifier::Identifier(Rc::new(str), Rc::new(Type::Undefined));
        while peek_match_ignore_ws!(self, 0, Token::Period) {
            self.skip_empty();
            self.next();
            let rhs = expect!(self, true, Token::Ident(str) => Ok(Identifier::Identifier(Rc::new(str), Rc::new(Type::Undefined))))?;
            ident = Identifier::Accessor(Box::new(ident), Box::new(rhs));
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
            self.skip_empty();
            self.next();
            self.ws_delim_in_parenth = true;
            let result = self.parse_math_expr()?;
            self.ws_delim_in_parenth = false;
            expect!(self, true, Token::ParenthClose => Ok(()))?;
            return Ok(result);
        }

        self.parse_unary_expr()
    }

    fn parse_unary_expr(&mut self) -> Result<MathExpr, ParserErr> {
        if peek_match_ignore_ws!(self, 0, Token::Subtraction) {
            self.skip_empty();
            self.next();
            Ok(MathExpr::Negate(Box::new(self.parse_parenth_expr()?)))
        } else {
            if peek_match_ignore_ws!(self, 0, Token::Number(_)) { 
                expect!(self, true, Token::Number(num) => Ok(MathExpr::Literal(num)))
            } else if peek_match_ignore_ws!(self, 0, Token::Ident(_)) {
                let ident = self.parse_ident()?;
                
                if self.parsing_ws_delim && !self.ws_delim_in_parenth {  
                    Ok(MathExpr::Identifier(ident))
                } else if let Some((Token::Whitespace, _)) = self.peek() { 
                    if peek_match_ignore_ws!(self, 1, Token::Addition, Token::Subtraction, Token::Multiplication, Token::Division, Token::Mod, Token::Power) {
                        self.next();
                        Ok(MathExpr::Identifier(ident))
                    } else { 
                        let args = self.parse_ws_delimited_exprs()?;
                        Ok(MathExpr::FuncCall(ident, args))
                    }
                } else { 
                    Ok(MathExpr::Identifier(ident))
                }
            } else { 
                Err(self.unexpected_token())
            } 
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
    
    fn unexpected_token(&mut self) -> ParserErr {
        let token = self.next().unwrap();
        
        ParserErr::UnexpectedToken {
            line: self.line,
            pos: token.1,
            token: token.0,
        }
    }
}
