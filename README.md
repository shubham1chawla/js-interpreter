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
    | Literal
    | Identifier
    ;

ParanthesizedExpression
    : '(' Expression ')'
    ;

FunctionExpression
    : 'function' OptIdentifier '(' OptFormalParameterList ')' BlockStatement
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
