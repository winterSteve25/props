use crate::error::ParserErr;
use crate::nodes::AstNode;

pub trait PropsSemanticAnalyzer {
    fn analyze(&self, ast: &Vec<AstNode>) -> Vec<ParserErr>;
}

pub struct PropsTypeChecker;
impl PropsSemanticAnalyzer for PropsTypeChecker {
    fn analyze(&self, ast: &Vec<AstNode>) -> Vec<ParserErr> {
        let errs = Vec::new();
        
        errs
    }
}