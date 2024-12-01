use parser::Parser;

mod parser;

fn main() {
    let content_string = "
        if (x) x += 42; else if (y) y += 42; else { x = 42; y = 10; }
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
