use thiserror::__private::AsDisplay;
use crate::types::typer::PropsTyper;
use crate::error::ParserErr;
use crate::nodes::AstNode;
use crate::parser::PropsParser;
use crate::types::TypeEnvironment;

pub struct PropsPipeline {
    parser: PropsParser,
    type_environment: TypeEnvironment,
    typer: PropsTyper,
}

impl PropsPipeline {
    pub fn new(parser: PropsParser) -> Self {
        let type_env = TypeEnvironment::new();
        
        PropsPipeline {
            parser,
            typer: PropsTyper,
            type_environment: type_env,
        }
    }
    
    pub fn parse(&mut self, source: String) -> (Vec<AstNode>, Vec<ParserErr>) {
        self.parser.init(source);
        self.type_environment.clear();
        
        let (ast, _) = self.parser.parse();
        let mut type_errs = vec![];
        self.typer.process(&ast, &mut self.type_environment, &mut type_errs);
        for err in type_errs.iter() {
            eprintln!("{}", err);
        }
        
        (ast, type_errs)
    }
}

impl Default for PropsPipeline {
    fn default() -> Self {
        PropsPipeline::new(PropsParser::new())
    }
}