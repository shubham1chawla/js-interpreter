use parser::{Parsable, Parser};

mod parser;

fn main() {
    let content_string = "
        for (let i=0, x = y; i<10; i+=1) {}
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
