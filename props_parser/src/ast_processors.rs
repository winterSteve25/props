use crate::error::ParserErr;
use crate::nodes::AstNode;

pub trait PropsAstProcessor {
    fn process(&self, ast: &mut Vec<AstNode>) -> Vec<ParserErr>;
}

pub struct PropsTyper;
impl PropsAstProcessor for PropsTyper {
    fn process(&self, ast: &mut Vec<AstNode>) -> Vec<ParserErr> {
        let errs = Vec::new();
        
        for node in ast {
            match node {
                AstNode::Assignment(_, _) => {}
                AstNode::ImpFuncCall(_, _) => {}
                _ => {}
            }
        }
        
        errs
    }
}