#[derive(Debug, PartialEq)]
pub enum Tree {
    Program{ body: Box<Vec<Tree>> },
    EmptyStatement,
    BlockStatement{ body: Box<Vec<Tree>> },
    ExpressionStatement{ expression: Box<Tree> },
    BinaryExpression{ operator: String, left: Box<Tree>, right: Box<Tree> },
    NumericLiteral{ value: f64 },
    StringLiteral{ value: String },
}
