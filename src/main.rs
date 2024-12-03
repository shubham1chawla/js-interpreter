use parser::{Parsable, Parser};

mod parser;

fn main() {
    let content_string = "
        let str, x = 0;
        if (x > 0) {
            str = 'Hello!';
        } else {
            str = 'World!';
        }
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
