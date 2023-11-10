use std::fs;

fn main() {

    let source_content = fs::read_to_string("../examples/example.prop")
        .expect("Err to read example source");

    println!("{:?}", props_parser::lexer::lex(source_content));
}
