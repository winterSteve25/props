use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use crate::nodes::{Expression, MathExpr};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64)
}

impl Number {
    pub fn parse_number(s: &str, has_decimal: bool) -> Result<Number, &'static str> {
        if has_decimal {
            if let Ok(value) = s.parse::<f32>() {
                Ok(Number::F32(value))
            } else if let Ok(value) = s.parse::<f64>() {
                Ok(Number::F64(value))
            } else {
                Err("Failed to parse the string into a valid number type.")
            }
        } else {
            if let Ok(value) = s.parse::<u8>() {
                Ok(Number::U8(value))
            } else if let Ok(value) = s.parse::<u16>() {
                Ok(Number::U16(value))
            } else if let Ok(value) = s.parse::<u32>() {
                Ok(Number::U32(value))
            } else if let Ok(value) = s.parse::<u64>() {
                Ok(Number::U64(value))
            } else if let Ok(value) = s.parse::<i8>() {
                Ok(Number::I8(value))
            } else if let Ok(value) = s.parse::<i16>() {
                Ok(Number::I16(value))
            } else if let Ok(value) = s.parse::<i32>() {
                Ok(Number::I32(value))
            } else if let Ok(value) = s.parse::<i64>() {
                Ok(Number::I64(value))
            } else {
                Err("Failed to parse the string into a valid number type.")
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Number::U8(num) => num.to_string().len(),
            Number::U16(num) => num.to_string().len(),
            Number::U32(num) => num.to_string().len(),
            Number::U64(num) => num.to_string().len(),
            Number::I8(num) => num.to_string().len(),
            Number::I16(num) => num.to_string().len(),
            Number::I32(num) => num.to_string().len(),
            Number::I64(num) => num.to_string().len(),
            Number::F32(num) => num.to_string().len(),
            Number::F64(num) => num.to_string().len(),
        }
    }
}

impl FromStr for Number {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let has_decimal = s.contains('.');
        Number::parse_number(s, has_decimal)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Undefined,
    Function(Box<Type>),
    Defined(String),
    Compound(Vec<Type>),
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        Type::Defined(value)
    }
}

#[derive(Debug)]
pub(crate) struct TypeEnvironment {
    types: HashMap<Rc<String>, Rc<Type>>
}

impl TypeEnvironment {
    pub fn new() -> Self {
        TypeEnvironment {
            types: HashMap::new()
        }
    }
    
    pub fn clear(&mut self) {
        
    }
    
    pub fn assign(&mut self, ident: Rc<String>, type_: Rc<Type>) {
        self.types.insert(ident, type_);
    }
    
    pub fn predict_type(&self, expr: &Expression) -> Type {
        // match expr {
        //     Expression::MathExpr(expr) => match expr {
        //         MathExpr::Literal(literal) => Type::
        //         MathExpr::Identifier(_) => {}
        //         MathExpr::BinaryOp(_, _, _) => {}
        //         MathExpr::Negate(_) => {}
        //         MathExpr::FuncCall(_, _) => {}
        //     }
        //     Expression::StrLiteral(_) => {}
        //     Expression::Compound(_) => {}
        //     Expression::FuncLiteral { .. } => {}
        // }
        
        Type::Undefined
    }
}