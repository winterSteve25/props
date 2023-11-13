use std::fs;

use props_parser::PropsParser;

fn main() {

    let source_content = fs::read_to_string("./examples/example.prop")
        .expect("Err to read example source");

    let mut parser = PropsParser::new(source_content);
    let result = parser.parse();
    
    println!("{:?}", result);
}
