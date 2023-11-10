#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AstNode {
    Assignment {
        names: Vec<String>,
        expr: Expression,
    },
    ImpFuncCall {
        name: String,
        expr: Expression
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expression {
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
