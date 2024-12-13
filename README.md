# Javascript Parser

This repository contains code to parse Javscript files, following tutorial by [Dmitry Soshnikov](http://dmitrysoshnikov.com/courses/parser-from-scratch/).

## Grammar

The following snippets highlight the grammar production rules based on [Backus–Naur form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).

```
Program
    : StatementList
    ;

StatementList
    : Statement
    | StatementList Statement
    ;

Statement
    : IterationStatement
    | FunctionDeclaration
    | ReturnKeyword
    | EmptyStatement
    | BlockStatement
    | VariableStatement
    | IfStatement
    | ClassDeclaration
    | ExpressionStatement
    ;

IterationStatement
    : WhileStatement
    | DoWhileStatement
    | ForStatement
    ;

WhileStatement
    : 'while' '(' Expression ')' Statement
    ;

DoWhileStatement
    : 'do' Statement '(' Expression ')' ';'
    ;

ForStatement
    : 'for' '(' OptForStatementInit ';' OptExpression ';' OptExpression ')' Statement
    ;

ForStatementInit
    : VariableStatementInit
    | Expression
    ;

FunctionDeclaration
    : 'function' Identifier '(' OptFormalParameterList ')' BlockStatement
    ;

FormalParameterList
    : Identifier
    | FormalParameterList ',' Identifier
    ;

ReturnStatement
    : 'return' OptExpression ';'
    ;

EmptyStatement
    : ';'
    ;

BlockStatement
    : '{' OptStatementList '}'
    ;

VariableStatement
    : VariableStatementInit ';'
    ;

VariableStatementInit
    : 'let' VariableDeclarationList
    ;

VariableDeclarationList
    : VariableDeclaration
    | VariableDeclarationList ',' VariableDeclaration
    ;

VariableDeclaration
    : Identifier OptVariableInitializer
    ;

VariableInitializer
    : SIMPLE_ASSIGNMENT_OPERATOR AssignmentExpression
    ;

IfStatement
    : 'if' '(' Expression ')' Statement
    | 'if' '(' Expression ')' Statement 'else' Statement
    ;

ClassDeclaration
    : 'class' Identifier OptClassExtends ClassBody
    ;

ClassExtends
    : 'extends' Identifier
    ;

ClassBody
    : '{' OptClassStatementList '}'
    ;

ClassStatementList
    : ClassStatement
    | ClassStatementList ClassStatement
    ;

ClassStatement
    : ConstructorDefinition
    | GetterDefinition
    | SetterDefinition
    | MethodDefinition
    | PropertyDefinition
    ;

ConstructorDefinition
    : 'constructor' '(' OptFormalParameterList ')' BlockStatement
    ;

GetterDefinition
    : 'get' Identifier '(' ')' BlockStatement
    ;

SetterDefinition
    : 'set' Identifier '(' Identifier ')' BlockStatement 
    ;

MethodDefinition
    : Identifier '(' OptFormalParameterList ')' BlockStatement
    ;

PropertyDefinition
    : Identifier OptPropertyInitializer ';'
    ;

PropertyInitializer
    : SIMPLE_ASSIGNMENT_OPERATOR AssignmentExpression
    ;

ExpressionStatement
    : Expression ';'
    ;

Expression
    : AssignmentExpression
    ;

AssignmentExpression
    : LogicalOrExpression
    | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
    ;

LogicalOrExpression
    : LogicalAndExpression
    | LogicalAndExpression '||' LogicalAndExpression
    ;

LogicalAndExpression
    : EqualityExpression
    | EqualityExpression '&&' EqualityExpression
    ;

EqualityExpression
    : RelationalExpression EQUALITY_OPERATOR EqualityExpression
    | RelationalExpression
    ;

RelationalExpression
    : AdditiveExpression
    | AdditiveExpression RELATIONAL_OPERATOR RelationalExpression
    ;

AdditiveExpression
    : MultiplicativeExpression
    | AdditiveExpression ADDITIVE_OPERATOR MultiplicativeExpression
    ;

MultiplicativeExpression
    : UnaryExpression
    | MultiplicativeExpression MULTIPLICATIVE_OPERATOR UnaryExpression
    ;

UnaryExpression
    : LeftHandSideExpression
    | ADDITIVE_OPERATOR UnaryExpression
    | LOGICAL_NOT UnaryExpression
    ;

LeftHandSideExpression
    : CallMemberExpression
    ;

CallMemberExpression
    : MemberExpression
    | CallExpression
    ;

CallExpression
    : Callee Arguments
    ;

Callee
    : MemberExpression
    | CallExpression
    ;

Arguments
    : '(' OptArgumentList ')'
    ;

ArgumentList
    : AssignmentExpression
    | ArgumentList ',' AssignmentExpression
    ;

MemberExpression
    : PrimaryExpression
    | MemberExpression '.' Identifier
    | MemberExpression '[' Expression ']'
    ;

PrimaryExpression
    : ParanthesizedExpression
    | FunctionExpression
    | NewExpression
    | Literal
    | Identifier
    ;

ParanthesizedExpression
    : '(' Expression ')'
    ;

FunctionExpression
    : 'function' OptIdentifier '(' OptFormalParameterList ')' BlockStatement
    ;

NewExpression
    : 'new' MemberExpression Arguments
    ;

Literal
    : NumericLiteral
    | StringLiteral
    | BooleanLiteral
    | NullLiteral
    | ThisLiteral
    | SuperLiteral
    ;

NumericLiteral
    : NUMBER
    ;

StringLiteral
    : STRING
    ;

BooleanLiteral
    : 'true'
    | 'false'
    ;

NullLiteral
    : 'null'
    ;

ThisLiteral
    : 'this'
    ;

SuperLiteral
    : 'super'
    ;

Identifier
    : IDENTIFIER
    ;
```
