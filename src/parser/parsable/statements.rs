use expressions::AssignmentExpressionParsable;
use identifier::IdentifierParsable;

use super::*;

pub trait StatementListParsable {
    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_lookahead_type: TokenType) -> Result<Vec<Tree>, SyntaxError>;

    /**
     * Statement
     *  : EmptyStatement
     *  | BlockStatement
     *  | VariableStatement
     *  | IfStatement
     *  | ExpressionStatement
     *  ;
     */
    fn statement(&mut self) -> Result<Tree, SyntaxError>;
}

pub trait EmptyStatementParsable {
    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    fn empty_statement(&mut self) -> Result<Tree, SyntaxError>;
}

pub trait BlockStatementParsable {
    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    fn block_statement(&mut self) -> Result<Tree, SyntaxError>;
}

pub trait VariableStatementParsable {
    /**
     * VariableStatement
     *  : 'let' VariableDeclarationList ';'
     *  ;
     */
    fn variable_statement(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * VariableDeclarationList
     *  : VariableDeclaration
     *  | VariableDeclarationList ',' VariableDeclaration
     *  ;
     */
    fn variable_declaration_list(&mut self) -> Result<Vec<Tree>, SyntaxError>;

    /**
     * VariableDeclaration
     *  : Identifier OptVariableInitializer
     *  ;
     */
    fn variable_declaration(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * VariableInitializer
     *  : SIMPLE_ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn variable_initializer(&mut self) -> Result<Tree, SyntaxError>;
}

pub trait IfStatementParsable {
    /**
     * IfStatement
     *  : 'if' '(' Expression ')' Statement
     *  | 'if' '(' Expression ')' Statement 'else' Statement
     *  ;
     */
    fn if_statement(&mut self) -> Result<Tree, SyntaxError>;
}

pub trait ExpressionStatementParsable {
    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * Expression
     *  : AssignmentExpression
     *  ;
     */
    fn expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl StatementListParsable for Parser {
    fn statement_list(&mut self, stop_lookahead_type: TokenType) -> Result<Vec<Tree>, SyntaxError> {
        let mut statement_list = vec![];

        while self.lookahead.token_type != stop_lookahead_type {
            statement_list.push(self.statement()?);
        }

        Ok(statement_list)
    }

    fn statement(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::SemiColon => self.empty_statement(),
            TokenType::CurlyBracketOpen => self.block_statement(),
            TokenType::LetKeyword => self.variable_statement(),
            TokenType::IfKeyword => self.if_statement(),
            _ => self.expression_statement(),
        }
    }
}

impl EmptyStatementParsable for Parser {
    fn empty_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::EmptyStatement)
    }
}

impl BlockStatementParsable for Parser {
    fn block_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::CurlyBracketOpen)?;
        let body = match self.lookahead.token_type {
            TokenType::CurlyBracketClose => vec![],
            _ => self.statement_list(TokenType::CurlyBracketClose)?,
        };
        self.eat(TokenType::CurlyBracketClose)?;
        Ok(Tree::BlockStatement { body: Box::new(body) })
    }
}

impl VariableStatementParsable for Parser {
    fn variable_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::LetKeyword)?;
        let declarations = self.variable_declaration_list()?;
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::VariableStatement { declarations: Box::new(declarations) })
    }

    fn variable_declaration_list(&mut self) -> Result<Vec<Tree>, SyntaxError> {
        let mut declarations = vec![self.variable_declaration()?];

        while self.lookahead.token_type == TokenType::Comma {
            self.eat(TokenType::Comma)?;
            declarations.push(self.variable_declaration()?);
        }

        Ok(declarations)
    }

    fn variable_declaration(&mut self) -> Result<Tree, SyntaxError> {
        let identifier = self.identifier()?;

        // OptVariableInitializer
        let init = match self.lookahead.token_type {
            TokenType::Comma | TokenType::SemiColon => None,
            _ => Some(self.variable_initializer()?),
        };

        Ok(Tree::VariableDeclaration { 
            identifier: Box::new(identifier), 
            init: Box::new(init),
        })
    }

    fn variable_initializer(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::SimpleAssignmentOperator)?;
        self.assignment_expression()
    }
}

impl IfStatementParsable for Parser {
    fn if_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::IfKeyword)?;

        self.eat(TokenType::CircleBracketOpen)?;
        let test = self.expression()?;
        self.eat(TokenType::CircleBracketClose)?;

        let consequent = self.statement()?;
        let alternate = match self.lookahead.token_type {
            TokenType::ElseKeyword => {
                self.eat(TokenType::ElseKeyword)?;
                Some(self.statement()?)
            },
            _ => None,
        };

        Ok(Tree::IfStatement {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternate: Box::new(alternate),
        })
    }
}

impl ExpressionStatementParsable for Parser {
    fn expression_statement(&mut self) -> Result<Tree, SyntaxError> {
        let expression = self.expression()?;
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::ExpressionStatement { expression: Box::new(expression) })
    }

    fn expression(&mut self) -> Result<Tree, SyntaxError> {
        self.assignment_expression()
    }
}

#[cfg(test)]
mod tests {
    use parsable::tests::{assert_syntax_error, assert_tree};

    use super::*;

    #[test]
    fn test_parse_statement_list() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]),
        };
        assert_tree(expected, "42;\"Hello\";");
    }

    #[test]
    fn test_parse_simple_empty_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::EmptyStatement,
            ]),
        };
        assert_tree(expected, ";");
    }

    #[test]
    fn test_parse_empty_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::EmptyStatement,
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::EmptyStatement,
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]),
        };
        assert_tree(expected, ";\n42;\n;\n'Hello';");
    }

    #[test]
    fn test_parse_empty_block() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::BlockStatement { body: Box::new(vec![]) }
            ]), 
        };
        assert_tree(expected, "{}");
    }

    #[test]
    fn test_parse_block_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::BlockStatement { 
                    body: Box::new(vec![
                        Tree::ExpressionStatement { 
                            expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                        }
                    ]) 
                },
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]), 
        };
        assert_tree(expected, "42; { //Commenting 42 -> 42;\n 'Hello'; } \"Hello\";");
    }

    #[test]
    fn test_parse_nested_block_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::BlockStatement { 
                    body: Box::new(vec![
                        Tree::BlockStatement { 
                            body: Box::new(vec![
                                Tree::ExpressionStatement { 
                                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                                },
                                Tree::BlockStatement { 
                                    body: Box::new(vec![
                                        Tree::ExpressionStatement { 
                                            expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                                        }
                                    ]) 
                                },
                            ]) 
                        },
                        Tree::BlockStatement { 
                            body: Box::new(vec![]) 
                        },
                    ]) 
                },
            ]), 
        };
        assert_tree(expected, "{{ 42; { 'Hello'; } }{}}");
    }

    #[test]
    fn test_parse_invalid_block_statement() {
        let expected = SyntaxError {
            message: String::from("Unexpected token EOF, expected Identifier!"),
        };
        assert_syntax_error(expected, "{");
    }

    #[test]
    fn test_parse_simple_no_init_variable_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("y") }),
                            init: Box::new(None),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let y;");
    }

    #[test]
    fn test_parse_simple_variable_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("str") }),
                            init: Box::new(Some(Tree::StringLiteral { value: String::from("Hello") })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let str = 'Hello';");
    }

    #[test]
    fn test_parse_multiple_no_init_variable_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("a") }),
                            init: Box::new(None),
                        },
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("b") }),
                            init: Box::new(None),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let a, b;");
    }

    #[test]
    fn test_parse_multiple_variable_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("c") }),
                            init: Box::new(None),
                        },
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("d") }),
                            init: Box::new(Some(Tree::NumericLiteral { value: 42.0 })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let c, d = 42;");
    }

    #[test]
    fn test_parse_chained_variable_statement_1() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("x") }),
                            init: Box::new(Some(Tree::AssignmentExpression {
                                operator: String::from("="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                            })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let x = y = 42;");
    }

    #[test]
    fn test_parse_chained_variable_statement_2() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("x") }),
                            init: Box::new(Some(Tree::AssignmentExpression {
                                operator: String::from("-="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                            })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let x = y -= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_operator_variable_statement() {
        let expected = SyntaxError {
            message: String::from("Unexpected token ComplexAssignmentOperator, expected SimpleAssignmentOperator!"),
        };
        assert_syntax_error(expected, "let x *= 42;");
    }

    #[test]
    fn test_parse_literal_eq_literal_variable_statement() {
        let expected = SyntaxError {
            message: String::from("Unexpected token Number, expected Identifier!"),
        };
        assert_syntax_error(expected, "let 42 = 42;");
    }

    #[test]
    fn test_parse_no_alternate_simple_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::Identifier { name: String::from("x") }),
                    consequent: Box::new(Tree::BlockStatement { body: Box::new(vec![]) }),
                    alternate: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "if (x) {}");
    }

    #[test]
    fn test_parse_simple_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::Identifier { name: String::from("x") }),
                    consequent: Box::new(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("+="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 1.0 })
                        }),
                    }),
                    alternate: Box::new(Some(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 42.0 })
                        }),
                    })),
                },
            ]),
        };
        assert_tree(expected, "if (x) x += 1; else x = 42;");
    }

    #[test]
    fn test_parse_chained_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::Identifier { name: String::from("x") }),
                    consequent: Box::new(Tree::IfStatement {
                        test: Box::new(Tree::Identifier { name: String::from("y") }),
                        consequent: Box::new(Tree::ExpressionStatement {
                            expression: Box::new(Tree::AssignmentExpression {
                                operator: String::from("+="),
                                left: Box::new(Tree::Identifier { name: String::from("x") }),
                                right: Box::new(Tree::Identifier { name: String::from("y") })
                            }),
                        }),
                        alternate: Box::new(Some(Tree::ExpressionStatement {
                            expression: Box::new(Tree::AssignmentExpression {
                                operator: String::from("="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 })
                            }),
                        })),
                    }),
                    alternate: Box::new(Some(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 10.0 })
                        }),
                    })),
                },
            ]),
        };
        assert_tree(expected, "if (x) if (y) x += y; else y = 42; else x = 10;");
    }

    #[test]
    fn test_parse_else_if_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::Identifier { name: String::from("x") }),
                    consequent: Box::new(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("+="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 42.0 })
                        }),
                    }),
                    alternate: Box::new(Some(Tree::IfStatement {
                        test: Box::new(Tree::Identifier { name: String::from("y") }),
                        consequent: Box::new(Tree::ExpressionStatement {
                            expression: Box::new(Tree::AssignmentExpression {
                                operator: String::from("+="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 })
                            }),
                        }),
                        alternate: Box::new(Some(Tree::BlockStatement {
                            body: Box::new(vec![
                                Tree::ExpressionStatement {
                                    expression: Box::new(Tree::AssignmentExpression {
                                        operator: String::from("="),
                                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                                        right: Box::new(Tree::NumericLiteral { value: 42.0 })
                                    }),
                                },
                                Tree::ExpressionStatement {
                                    expression: Box::new(Tree::AssignmentExpression {
                                        operator: String::from("="),
                                        left: Box::new(Tree::Identifier { name: String::from("y") }),
                                        right: Box::new(Tree::NumericLiteral { value: 10.0 })
                                    }),
                                }
                            ]),
                        })),
                    })),
                },
            ]),
        };
        assert_tree(expected, "if (x) x += 42; else if (y) y += 42; else { x = 42; y = 10; }");
    }
}