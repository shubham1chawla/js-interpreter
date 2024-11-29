use parser::Parser;

mod parser;

fn main() {
    let content_string = "x += y + 10;";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
