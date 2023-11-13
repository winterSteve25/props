use crate::tokens::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Assignment {
        name: Identifier,
        expr: Expression,
    },
    ImpFuncCall {
        name: Expression,
        expr: Expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    MathExpr(MathExpr),
    StrLiteral(String),
    FuncLiteral {
        params: Vec<String>,
        statements: Vec<AstNode>
    },
    FuncCall {
        func_name: String,
        params: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Identifier(String),
    Accessor(String, Box<Identifier>),
    Compound(Vec<Identifier>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    Literal(Number),
    Add(Box<MathExpr>, Box<MathExpr>),
    Sub(Box<MathExpr>, Box<MathExpr>),
    Mul(Box<MathExpr>, Box<MathExpr>),
    Div(Box<MathExpr>, Box<MathExpr>),
    Mod(Box<MathExpr>, Box<MathExpr>),
    Pow(Box<MathExpr>, Box<MathExpr>),
}
