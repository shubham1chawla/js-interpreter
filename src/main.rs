use crate::prelude::*;

mod interpreter;
mod parser;
mod environment;
mod errors;
mod prelude;
mod tokenizer;
mod tree;
mod value;

fn main() {
    let content_string = "
        let x = null, y = 'Hello', z = 10 * (5 - '2');
        x;
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let result = parser.parse();
    let tree = result.unwrap();

    let interpreter = Interpreter::new(tree);
    let result = interpreter.eval();
    println!("{result:?}");
}
