use std::rc::Rc;
use crate::error::ParserErr;
use crate::nodes::{AstNode, Expression, Identifier};
use crate::types::{Type, TypeEnvironment};

pub(crate) struct PropsTyper;
impl PropsTyper {
    pub fn process(&self, ast: &Vec<AstNode>, type_environment: &mut TypeEnvironment) {
        for node in ast {
            if let AstNode::Assignment(ident, expr) = node {
                self.assign_type(ident, type_environment, expr);
            } 
        }
    }
    
    fn assign_type(&self, ident: &Identifier, type_environment: &mut TypeEnvironment, expr: &Expression) {
        match ident {
            Identifier::Identifier(str, t) => {
                match **t {
                    Type::Undefined => type_environment.assign(str.clone(), Rc::new(type_environment.predict_type(expr))),
                    _ => type_environment.assign(str.clone(), t.clone())
                }
            },
            Identifier::Compound(idents) => for ident in idents.iter() {
                self.assign_type(ident, type_environment, expr);
            },
            _ => {}
        }
    }
}