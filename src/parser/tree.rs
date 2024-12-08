#[derive(Debug, PartialEq)]
pub enum Tree {
    
    /**
     * Program
     *  : StatementList
     *  ;
     * 
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     * 
     * Statement
     *  : IterationStatement
     *  | EmptyStatement
     *  | BlockStatement
     *  | VariableStatement
     *  | IfStatement
     *  | ExpressionStatement
     *  ;
     * 
     * IterationStatement
     *  : WhileStatement
     *  | DoWhileStatement
     *  | ForStatement
     *  ;
     */
    Program{ body: Box<Vec<Tree>> },

    /**
     * WhileStatement
     *  : 'while' '(' Expression ')' Statement
     *  ;
     */
    WhileStatement{ test: Box<Tree>, body: Box<Tree> },
    
    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    EmptyStatement,

    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    BlockStatement{ body: Box<Vec<Tree>> },

    /**
     * VariableStatement
     *  : 'let' VariableDeclarationList ';'
     *  ;
     * 
     * VariableDeclarationList
     *  : VariableDeclaration
     *  | VariableDeclarationList ',' VariableDeclaration
     *  ;
     */
    VariableStatement{ declarations: Box<Vec<Tree>> },

    /**
     * VariableDeclaration
     *  : Identifier OptVariableInitializer
     *  ;
     * 
     * VariableInitializer
     *  : SIMPLE_ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    VariableDeclaration{ identifier: Box<Tree>, init: Box<Option<Tree>> },

    /**
     * IfStatement
     *  : 'if' '(' Expression ')' Statement
     *  | 'if' '(' Expression ')' Statement 'else' Statement
     *  ;
     */
    IfStatement{ test: Box<Tree>, consequent: Box<Tree>, alternate: Box<Option<Tree>> },

    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     * 
     * Expression
     *  : AssignmentExpression
     *  ;
     */
    ExpressionStatement{ expression: Box<Tree> },

    /**
     * AssignmentExpression
     *  : LogicalOrExpression
     *  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    AssignmentExpression{ operator: String, left: Box<Tree>, right: Box<Tree> },

    /**
     * LogicalOrExpression
     *  : LogicalAndExpression
     *  | LogicalAndExpression '||' LogicalAndExpression
     *  ;
     * 
     * LogicalAndExpression
     *  : EqualityExpression
     *  | EqualityExpression '&&' EqualityExpression
     *  ;
     */
    LogicalExpression{ operator: String, left: Box<Tree>, right: Box<Tree> },

    /**
     * EqualityExpression
     *  : RelationalExpression EQUALITY_OPERATOR EqualityExpression
     *  | RelationalExpression
     *  ;
     * 
     * RelationalExpression
     *  : AdditiveExpression
     *  | AdditiveExpression RELATIONAL_OPERATOR RelationalExpression
     *  ;
     * 
     * AdditiveExpression
     *  : MultiplicativeExpression
     *  | AdditiveExpression ADDITIVE_OPERATOR MultiplicativeExpression
     *  ;
     * 
     * MultiplicativeExpression
     *  : UnaryExpression
     *  | MultiplicativeExpression MULTIPLICATIVE_OPERATOR UnaryExpression
     *  ;
     */
    BinaryExpression{ operator: String, left: Box<Tree>, right: Box<Tree> },

    /**
     * UnaryExpression
     *  : LeftHandSideExpression
     *  | ADDITIVE_OPERATOR UnaryExpression
     *  | LOGICAL_NOT UnaryExpression
     *  ;
     * 
     * LeftHandSideExpression
     *  : PrimaryExpression
     *  ;
     * 
     * PrimaryExpression
     *  : ParanthesizedExpression
     *  | Literal
     *  | Identifier
     *  ;
     * 
     * ParanthesizedExpression
     *  : '(' Expression ')'
     *  ;
     * 
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    UnaryExpression{ operator: String, argument: Box<Tree> },

    /**
     * Identifier
     *  : IDENTIFIER
     *  ;
     */
    Identifier{ name: String },

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    NumericLiteral{ value: f64 },

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    StringLiteral{ value: String },

    /**
     * BooleanLiteral
     *  : 'true'
     *  | 'false'
     *  ;
     */
    BooleanLiteral{ value: bool },

    /**
     * NullLiteral
     *  : 'null'
     *  ;
     */
    NullLiteral,
}
