use crate::types::{Number, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Assignment {
        name: Identifier,
        expr: Expression,
    },
    ImpFuncCall {
        name: Identifier,
        arguments: Vec<Expression>
    },
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
    FuncCall {
        func_name: Identifier,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Identifier(String, Option<Type>),
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