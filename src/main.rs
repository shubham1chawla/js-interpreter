use parser::Parser;

mod parser;

fn main() {
    let content_string = "
        y = (x + 10) * 3 > 100;
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
