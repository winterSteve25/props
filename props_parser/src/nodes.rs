use std::rc::Rc;
use crate::types::{Number, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Assignment(Identifier, Expression),
    ImpFuncCall(Identifier, Vec<Expression>),
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    MathExpr(MathExpr),
    StrLiteral(String),
    Compound(Vec<Expression>),
    FuncLiteral {
        params: Vec<(String, Type)>,
        statements: Vec<AstNode>,
        return_type: Type,
    },
}

impl Expression {
    pub fn compound(self, other: Expression) -> Expression {
        if let Expression::Compound(mut v) = self {
            v.push(other);
            return Expression::Compound(v);
        }

        Expression::Compound(vec![self, other])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Identifier(Rc<String>, Rc<Type>),
    Accessor(Box<Identifier>, Box<Identifier>),
    Compound(Vec<Identifier>),
}

impl Identifier {
    pub fn compound(self, other: Identifier) -> Identifier {
        if let Identifier::Compound(mut v) = self {
            v.push(other);
            return Identifier::Compound(v);
        }
        
        Identifier::Compound(vec![self, other])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    Literal(Number),
    Identifier(Identifier),
    BinaryOp(Box<MathExpr>, Box<MathExpr>, MathOp),
    Negate(Box<MathExpr>),
    FuncCall(Identifier, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}