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
     *  : EmptyStatement
     *  | BlockStatement
     *  | VariableStatement
     *  | IfStatement
     *  | ExpressionStatement
     *  ;
     */
    Program{ body: Box<Vec<Tree>> },
    
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
     * 
     * LeftHandSideExpression
     *  : Identifier
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
     *  : PrimaryExpression
     *  | MultiplicativeExpression MULTIPLICATIVE_OPERATOR PrimaryExpression
     *  ;
     * 
     * PrimaryExpression
     *  : Literal
     *  | ParanthesizedExpression
     *  | LeftHandSideExpression
     *  ;
     * 
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     * 
     * ParanthesizedExpression
     *  : '(' Expression ')'
     *  ;
     */
    BinaryExpression{ operator: String, left: Box<Tree>, right: Box<Tree> },

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
