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
    : EmptyStatement
    | BlockStatement
    | VariableStatement
    | IfStatement
    | ExpressionStatement
    ;

EmptyStatement
    : ';'
    ;
    
BlockStatement
    : '{' OptStatementList '}'
    ;

VariableStatement
    : 'let' VariableDeclarationList ';'
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

ExpressionStatement
    : Expression ';'
    ;
    
Expression
    : AssignmentExpression
    ;

AssignmentExpression
    : EqualityExpression
    | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
    ;
    
LeftHandSideExpression
    : Identifier
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
    : PrimaryExpression
    | MultiplicativeExpression MULTIPLICATIVE_OPERATOR PrimaryExpression
    ;
    
PrimaryExpression
    : Literal
    | ParanthesizedExpression
    | LeftHandSideExpression
    ;
    
ParanthesizedExpression
    : '(' Expression ')'
    ;

Identifier
    : IDENTIFIER
    ;

Literal
    : NumericLiteral
    | StringLiteral
    | BooleanLiteral
    | NullLiteral
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
```
