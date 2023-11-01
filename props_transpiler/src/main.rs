use std::fs;

fn main() {

    let source_content = fs::read_to_string("../examples/example.prop")
        .expect("Err to read example source");

    match props_parser::parse(source_content) {
        Ok(tree) => println!("{:?}", tree),
        Err(err) => println!("{:?}", err)
    }
}
