use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use lazy_static::lazy_static;
use strum::{Display, EnumIter};
use strum::IntoEnumIterator;
use crate::nodes::{Expression, Identifier, MathExpr};
use crate::util::Access;

pub mod typer;

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
    F64(f64),
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

    pub fn prim_type(&self) -> PrimitiveType {
        // unless denoted default to integer over unsigned
        match self {
            Number::U8(_) => PrimitiveType::I16,
            Number::U16(_) => PrimitiveType::I32,
            Number::U32(_) => PrimitiveType::I64,
            Number::U64(_) => PrimitiveType::U64,
            Number::I8(_) => PrimitiveType::I32,
            Number::I16(_) => PrimitiveType::I32,
            Number::I32(_) => PrimitiveType::I32,
            Number::I64(_) => PrimitiveType::I64,
            Number::F32(_) => PrimitiveType::F32,
            Number::F64(_) => PrimitiveType::F64,
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
    Compound(Vec<Access<Type>>),
    Primitive(PrimitiveType),
}

lazy_static! {
    static ref PRIM_TYPES: HashMap<String, PrimitiveType> = {
        let mut map = HashMap::new();
        for t in PrimitiveType::iter() {
            map.insert(t.to_string(), t);
        }
        
        map
    };
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        match PRIM_TYPES.get(&value) {
            Some(prim) => Type::Primitive(prim.clone()),
            None => Type::Defined(value),
        }
    }
}

impl FromIterator<Access<Type>> for Type {
    fn from_iter<T: IntoIterator<Item=Access<Type>>>(iter: T) -> Self {
        let collection: Vec<Access<Type>> = iter
            .into_iter()
            .collect();
        Type::Compound(collection)
    }
}

impl PartialOrd for PrimitiveType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_size = match self {
            PrimitiveType::U8 | PrimitiveType::I8 => 1,
            PrimitiveType::U16 | PrimitiveType::I16 => 2,
            PrimitiveType::U32 | PrimitiveType::I32 | PrimitiveType::F32 => 4,
            PrimitiveType::U64 | PrimitiveType::I64 | PrimitiveType::F64 => 8,
            PrimitiveType::Str => return None
        };

        let other_size = match other {
            PrimitiveType::U8 | PrimitiveType::I8 => 1,
            PrimitiveType::U16 | PrimitiveType::I16 => 2,
            PrimitiveType::U32 | PrimitiveType::I32 | PrimitiveType::F32 => 4,
            PrimitiveType::U64 | PrimitiveType::I64 | PrimitiveType::F64 => 8,
            PrimitiveType::Str => return None
        };

        self_size.partial_cmp(&other_size)
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Type::Primitive(pt1), Type::Primitive(pt2)) => pt1.partial_cmp(pt2),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Display)]
pub enum PrimitiveType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Str,
}

#[derive(Debug)]
pub(crate) struct TypeEnvironment {
    types: HashMap<Access<String>, Access<Type>>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        TypeEnvironment {
            types: HashMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.types.clear();
    }

    pub fn assign(&mut self, ident: Access<String>, type_: Access<Type>) {
        self.types.insert(ident, type_);
    }

    pub fn predict_type(expr: &Expression) -> Access<Type> {
        match expr {
            Expression::MathExpr(expr) => TypeEnvironment::type_math_expr(expr),
            Expression::StrLiteral(_) => Access::Owned(Type::Primitive(PrimitiveType::Str)),
            Expression::Compound(c) => c.iter()
                .map(|expr| TypeEnvironment::predict_type(expr))
                .collect::<Type>()
                .into(),
            Expression::FuncLiteral { return_type, .. } => Access::Owned(Type::Function(Box::new(return_type.clone()))),
        }
    }

    fn type_ident(ident: &Identifier) -> Access<Type> {
        match ident {
            Identifier::Identifier(_, t) => Access::Rc(t.clone()),
            Identifier::Compound(vec) => vec.iter()
                .map(|id| TypeEnvironment::type_ident(id))
                .collect::<Type>()
                .into(),
            _ => todo!()
        }
    }

    fn type_math_expr(expr: &MathExpr) -> Access<Type> {
        match expr {
            MathExpr::Literal(literal) => Access::Owned(Type::Primitive(literal.prim_type())),
            MathExpr::Identifier(ident) => TypeEnvironment::type_ident(ident),
            MathExpr::BinaryOp(lhs, rhs, _) => {
                let lhs_type = TypeEnvironment::type_math_expr(lhs);
                let rhs_type = TypeEnvironment::type_math_expr(rhs);

                if lhs_type == rhs_type {
                    lhs_type
                } else if lhs_type < rhs_type {
                    rhs_type
                } else if lhs_type > rhs_type {
                    lhs_type
                } else {
                    todo!()
                }
            }
            MathExpr::Negate(expr) => TypeEnvironment::type_math_expr(expr),
            MathExpr::FuncCall(ident, _) => TypeEnvironment::type_ident(ident)
        }
    }
}
