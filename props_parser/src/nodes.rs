use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Statement {
    Assignment {
        names: Vec<String>,
        expr: Expression,
    },
    ExpectedToken {
        line: usize,
        column: usize,
        token: Token,
    },
    UnexpectedToken {
        line: usize,
        column: usize,
        token: Token,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expression {
    MathExpr(MathExpr),
    StrLiteral(String),
    FuncCall {
        func_name: String,
        params: Vec<Expression>,
    },
    Func {
        params: Vec<String>,
        statements: Vec<Statement>
    },
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum MathExpr {
    Number(i32),
    Add(Box<MathExpr>, Box<MathExpr>),
    Sub(Box<MathExpr>, Box<MathExpr>),
    Mul(Box<MathExpr>, Box<MathExpr>),
    Div(Box<MathExpr>, Box<MathExpr>),
    Mod(Box<MathExpr>, Box<MathExpr>),
    Pow(Box<MathExpr>, Box<MathExpr>),
}
