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
        let x = null, y = 'Hello', z = true;
        x;
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let result = parser.parse();
    let tree = result.unwrap();

    let mut env = EnvironmentBuilder::default().build().unwrap();
    let interpreter = Interpreter::new(&tree, &mut env);
    let result = interpreter.eval();
    println!("{result:?}");
}
