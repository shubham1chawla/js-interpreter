#[derive(Debug, PartialEq)]
pub enum Tree {
    Program{ body: Box<Vec<Tree>> },
    ExpressionStatement{ expression: Box<Tree> },
    NumericLiteral{ value: f64 },
    StringLiteral{ value: String },
}
