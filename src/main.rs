use parser::{Parsable, Parser};

mod parser;

fn main() {
    let content_string = "
        x.y;
        x.y = 1;
        x[0] = 1;
        a.b.c['d'];
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
