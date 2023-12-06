use crate::error::ParserErr;
use crate::nodes::{AstNode, Expression, Identifier};
use crate::types::{Type, TypeEnvironment};
use crate::util::Access;

pub(crate) struct PropsTyper;
impl PropsTyper {
    pub fn process(&self, ast: &Vec<AstNode>, type_environment: &mut TypeEnvironment, errs: &mut Vec<ParserErr>) {
        for node in ast {
            if let AstNode::Assignment(ident, expr) = node {
                self.assign_type(ident, type_environment, expr, errs);
            }
        }
    }

    fn assign_type(&self, ident: &Identifier, type_environment: &mut TypeEnvironment, expr: &Expression, errs: &mut Vec<ParserErr>) {
        let expr_type = TypeEnvironment::predict_type(expr);
        
        match ident {
            Identifier::Identifier(str, t) => match **t {
                Type::Undefined => type_environment.assign(str.into(), expr_type),
                _ => {
                    if expr_type.map(|v| v == t.as_ref()) {
                        type_environment.assign(str.into(), t.into());
                    }
                    
                    errs.push(ParserErr::UnmatchedTypes {
                        type_1: expr_type,
                        type_2: Access::Rc(t.clone()),
                    });
                }
            },
            Identifier::Compound(idents) => {
                if expr_type.map(|v| match v { Type::Compound(_) => true, _ => false }) { 
                    
                }
            }
            _ => todo!()
        }
    }
}