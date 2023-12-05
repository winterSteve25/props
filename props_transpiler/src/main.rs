use std::fs;
use props_parser::pipeline::PropsPipeline;

fn main() {

    // let mut parser = PropsParser::new("obj.field.josh, hi = \"josh\", 2".to_string());
    // let result = parser.parse();
    
    let source_content = fs::read_to_string("./examples/example.prop")
        .expect("Err to read example source");
    
    let mut parser = PropsPipeline::default();
    let result = parser.parse(source_content);
    
    println!("{:?}", result);
}
