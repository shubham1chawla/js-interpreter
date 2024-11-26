#[derive(Debug, PartialEq)]
pub enum TreeNodeType {
    Program,
    NumericLiteral,
    StringLiteral,
}

#[derive(Debug, PartialEq)]
pub enum Tree {
    Program{ node_type: TreeNodeType, body: Box<Tree> },
    NumericLiteral{ node_type: TreeNodeType, value: f64 },
    StringLiteral{ node_type: TreeNodeType, value: String },
}
