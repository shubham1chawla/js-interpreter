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
     * DoWhileStatement
     *  : 'do' Statement '(' Expression ')' ';'
     *  ;
     */
    DoWhileStatement{ body: Box<Tree>, test: Box<Tree> },

    /**
     * ForStatement
     *  : 'for' '(' OptForStatementInit ';' OptExpression ';' OptExpression ')' Statement
     *  ;
     * 
     * ForStatementInit
     *  : VariableStatementInit
     *  | Expression
     *  ;
     */
    ForStatement{ init: Box<Option<Tree>>, test: Box<Option<Tree>>, update: Box<Option<Tree>>, body: Box<Tree> },

    /**
     * FunctionDeclaration
     *  : 'function' Identifier '(' OptFormalParameterList ')' BlockStatement
     *  ;
     * 
     * FormalParameterList
     *  : Identifier
     *  | FormalParameterList ',' Identifier
     *  ;
     */
    FunctionDeclaration{ identifier: Box<Tree>, params: Box<Vec<Tree>>, body: Box<Tree> },

    /**
     * ReturnStatement
     *  : 'return' OptExpression ';'
     *  ;
     */
    ReturnStatement{ argument: Box<Option<Tree>> },
    
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
     *  : VariableStatementInit ';'
     *  ;
     * 
     * VariableStatementInit
     *  : 'let' VariableDeclarationList
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
     *  : CallMemberExpression
     *  ;
     */
    UnaryExpression{ operator: String, argument: Box<Tree> },

    /**
     * CallMemberExpression
     *  : MemberExpression
     *  | CallExpression
     *  ;
     * 
     * CallExpression
     *  : Callee Arguments
     *  ;
     * 
     * Callee
     *  : MemberExpression
     *  | CallExpression
     *  ;
     * 
     * Arguments
     *  : '(' OptArgumentList ')'
     *  ;
     * 
     * ArgumentList
     *  : AssignmentExpression
     *  | ArgumentList ',' AssignmentExpression
     *  ;
     */
    CallExpression{ callee: Box<Tree>, arguments: Box<Vec<Tree>> },

    /**
     * MemberExpression
     *  : PrimaryExpression
     *  | MemberExpression '.' Identifier
     *  | MemberExpression '[' Expression ']'
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
    MemberExpression{ object: Box<Tree>, property: Box<Tree>, computed: bool },

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
