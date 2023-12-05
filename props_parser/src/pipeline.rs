use crate::analysis::PropsTypeChecker;
use crate::ast_processors::PropsTyper;
use crate::error::ParserErr;
use crate::nodes::AstNode;
use crate::parser::PropsParser;
use crate::types::TypeEnvironment;

pub struct PropsPipeline {
    parser: PropsParser,
    type_environment: TypeEnvironment,
    typer: PropsTyper,
    type_checker: PropsTypeChecker,
}

impl PropsPipeline {
    pub fn new(parser: PropsParser) -> Self {
        let type_env = TypeEnvironment::new();
        
        PropsPipeline {
            parser,
            typer: PropsTyper,
            type_checker: PropsTypeChecker,
            type_environment: type_env,
        }
    }
    
    pub fn parse(&mut self, source: String) -> (Vec<AstNode>, Vec<ParserErr>) {
        self.parser.init(source);
        self.type_environment.clear();
        
        let (ast, mut errs) = self.parser.parse();
        
        self.typer.process(&ast, &mut self.type_environment);
        errs.append(&mut self.type_checker.analyze(&ast, &self.type_environment));
        
        (ast, errs)
    }
}

impl Default for PropsPipeline {
    fn default() -> Self {
        PropsPipeline::new(PropsParser::new())
    }
}