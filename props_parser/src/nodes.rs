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
    Identifier(Identifier),
    MathExpr(MathExpr),
    StrLiteral(String),
    FuncLiteral {
        params: Vec<(String, Type)>,
        statements: Vec<AstNode>,
        return_type: Type,
    },
    FuncCall(Identifier, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Identifier(String, Type),
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