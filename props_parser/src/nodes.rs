#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment(String, Option<Vec<String>>, Expression),
    ImpureStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    MathExpr(MathExpr),
    Func(Vec<String>, Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    Number(i32),
    Add(Box<MathExpr>, Box<MathExpr>),
    Sub(Box<MathExpr>, Box<MathExpr>),
    Mul(Box<MathExpr>, Box<MathExpr>),
    Div(Box<MathExpr>, Box<MathExpr>),
    Mod(Box<MathExpr>, Box<MathExpr>),
}
