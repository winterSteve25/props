use crate::analysis::PropsSemanticAnalyzer;
use crate::ast_processors::PropsAstProcessor;
use crate::parser::PropsParser;

struct PropsPipeline {
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
    
    pub fn parse(&self, content: &str) {
        
    }
}