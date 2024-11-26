use parser::Parser;

mod parser;

fn main() {
    let parser = Parser::new("42".to_owned());
    let node = parser.parse();
    println!("{:?}", node);
}
