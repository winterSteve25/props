use crate::error::ParserErr;
use crate::nodes::AstNode;
use crate::types::TypeEnvironment;

pub(crate) struct PropsTypeChecker;
impl PropsTypeChecker {
    pub fn analyze(&self, ast: &Vec<AstNode>, type_environment: &TypeEnvironment) -> Vec<ParserErr> {
        let errs = Vec::new();
        
        errs
    }
}