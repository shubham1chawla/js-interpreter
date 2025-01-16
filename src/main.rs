use crate::prelude::*;

mod interpreter;
mod parser;
mod errors;
mod prelude;
mod tokenizer;
mod tree;

fn main() {
    let content_string = "
        10;
        'Hello, World!';
        3 + 5;
        2 - 7;
        '2' - (1 + 4);
        '10' * ('2' - 3 + (5 - 2 + 1));
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let result = parser.parse();

    let interpreter = Interpreter::new();
    let result = interpreter.eval(result.unwrap());
    println!("{result:#?}");
}
