use crate::analysis::{PropsSemanticAnalyzer, PropsTypeChecker};
use crate::ast_processors::{PropsAstProcessor, PropsTyper};
use crate::error::ParserErr;
use crate::nodes::AstNode;
use crate::parser::PropsParser;

pub struct PropsPipeline {
    parser: PropsParser,
    ast_processors: Vec<Box<dyn PropsAstProcessor>>,
    analyzers: Vec<Box<dyn PropsSemanticAnalyzer>>,
}

impl PropsPipeline {
    pub fn new(parser: PropsParser) -> Self {
        PropsPipeline {
            parser,
            ast_processors: vec![],
            analyzers: vec![] 
        }
    }
    
    pub fn analyzer<T>(mut self, analyzer: T) -> Self
    where T : PropsSemanticAnalyzer + 'static
    {
        self.analyzers.push(Box::new(analyzer));
        self
    }
    
    pub fn processor<T>(mut self, processor: T) -> Self
    where T : PropsAstProcessor + 'static
    {
        self.ast_processors.push(Box::new(processor));
        self
    }
    
    pub fn parse(&mut self, source: String) -> (Vec<AstNode>, Vec<ParserErr>) {
        self.parser.init(source);
        
        let (mut ast, mut errs) = self.parser.parse();

        for ast_processor in self.ast_processors.iter() {
            let mut new_errs = ast_processor.process(&mut ast);
            errs.append(&mut new_errs);
        }
        
        for analyzer in self.analyzers.iter() {
            let mut new_errs = analyzer.analyze(&ast);
            errs.append(&mut new_errs);
        }

        (ast, errs)
    }
}

impl Default for PropsPipeline {
    fn default() -> Self {
        PropsPipeline::new(PropsParser::new())
            .processor(PropsTyper)
            .analyzer(PropsTypeChecker)
    }
}