#[derive(Debug, PartialEq)]
pub enum Tree {
    
    /**
     * Program {
     *  body: [...],
     * }
     */
    Program{ body: Box<Vec<Tree>> },
    
    /**
     * EmptyStatement
     */
    EmptyStatement,

    /**
     * BlockStatement {
     *  body: [...],
     * }
     */
    BlockStatement{ body: Box<Vec<Tree>> },

    /**
     * ExpressionStatement {
     *  expression: ...,
     * }
     */
    ExpressionStatement{ expression: Box<Tree> },

    /**
     * VariableStatement {
     *  declarations: [...],
     * }
     */
    VariableStatement{ declarations: Box<Vec<Tree>> },

    /**
     * VariableDeclaration {
     *  identifier: ...,
     *  init: ...,
     * }
     */
    VariableDeclaration{ identifier: Box<Tree>, init: Box<Option<Tree>> },

    /**
     * AssignmentExpression {
     *  operator: ...,
     *  left: ...,
     *  right: ...,
     * }
     */
    AssignmentExpression{ operator: String, left: Box<Tree>, right: Box<Tree> },

    /**
     * Identifier {
     *  name: ...,
     * }
     */
    Identifier{ name: String },

    /**
     * BinaryExpression {
     *  operator: ...,
     *  left: ...,
     *  right: ...,
     * }
     */
    BinaryExpression{ operator: String, left: Box<Tree>, right: Box<Tree> },

    /**
     * NumericLiteral {
     *  value: ...,
     * }
     */
    NumericLiteral{ value: f64 },

    /**
     * StringLiteral {
     *  value: ...,
     * }
     */
    StringLiteral{ value: String },
}
