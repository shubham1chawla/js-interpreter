use parser::{Parsable, Parser};

mod parser;

fn main() {
    let content_string = "
        function multiply(x, y) { return x * y; }
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
