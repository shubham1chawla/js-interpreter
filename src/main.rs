use parser::{Parsable, Parser};

mod parser;

fn main() {
    let content_string = "
        do x += 1; while (x < 42);
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
