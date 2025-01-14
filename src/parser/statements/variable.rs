use crate::prelude::*;

use super::expressions::assignment::AssignmentExpressionParsable;
use super::identifier::IdentifierParsable;

pub trait VariableStatementParsable {
    /**
     * VariableStatement
     *  : VariableStatementInit ';'
     *  ;
     */
    fn variable_statement(&mut self) -> Result<Tree>;

    /**
     * VariableStatementInit
     *  : 'let' VariableDeclarationList
     *  ;
     */
    fn variable_statement_init(&mut self) -> Result<Tree>;

    /**
     * VariableDeclarationList
     *  : VariableDeclaration
     *  | VariableDeclarationList ',' VariableDeclaration
     *  ;
     */
    fn variable_declaration_list(&mut self) -> Result<Vec<Tree>>;

    /**
     * VariableDeclaration
     *  : Identifier OptVariableInitializer
     *  ;
     */
    fn variable_declaration(&mut self) -> Result<Tree>;

    /**
     * VariableInitializer
     *  : SIMPLE_ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn variable_initializer(&mut self) -> Result<Tree>;
}

impl VariableStatementParsable for Parser {
    fn variable_statement(&mut self) -> Result<Tree> {
        let statement = self.variable_statement_init()?;
        self.eat(TokenType::SemiColon)?;
        Ok(statement)
    }

    fn variable_statement_init(&mut self) -> Result<Tree> {
        self.eat(TokenType::LetKeyword)?;
        let declarations = self.variable_declaration_list()?;
        Ok(Tree::VariableStatement { declarations })
    }

    fn variable_declaration_list(&mut self) -> Result<Vec<Tree>> {
        let mut declarations = vec![self.variable_declaration()?];

        while self.lookahead.token_type == TokenType::Comma {
            self.eat(TokenType::Comma)?;
            declarations.push(self.variable_declaration()?);
        }

        Ok(declarations)
    }

    fn variable_declaration(&mut self) -> Result<Tree> {
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

    fn variable_initializer(&mut self) -> Result<Tree> {
        self.eat(TokenType::SimpleAssignmentOperator)?;
        self.assignment_expression()
    }
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::tests::*;

    #[test]
    fn test_parse_simple_no_init_variable_statement() {
        let expected = Tree::Program { 
            body: vec![
                Tree::VariableStatement { 
                    declarations: vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("y") }),
                            init: Box::new(None),
                        },
                    ],
                },
            ],
        };
        assert_tree(expected, "let y;");
    }

    #[test]
    fn test_parse_simple_variable_statement() {
        let expected = Tree::Program { 
            body: vec![
                Tree::VariableStatement { 
                    declarations: vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("str") }),
                            init: Box::new(Some(Tree::StringLiteral { value: String::from("Hello") })),
                        },
                    ],
                },
            ],
        };
        assert_tree(expected, "let str = 'Hello';");
    }

    #[test]
    fn test_parse_multiple_no_init_variable_statement() {
        let expected = Tree::Program { 
            body: vec![
                Tree::VariableStatement { 
                    declarations: vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("a") }),
                            init: Box::new(None),
                        },
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("b") }),
                            init: Box::new(None),
                        },
                    ],
                },
            ],
        };
        assert_tree(expected, "let a, b;");
    }

    #[test]
    fn test_parse_multiple_variable_statement() {
        let expected = Tree::Program { 
            body: vec![
                Tree::VariableStatement { 
                    declarations: vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("c") }),
                            init: Box::new(None),
                        },
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("d") }),
                            init: Box::new(Some(Tree::NumericLiteral { value: 42.0 })),
                        },
                    ],
                },
            ],
        };
        assert_tree(expected, "let c, d = 42;");
    }

    #[test]
    fn test_parse_chained_variable_statement_1() {
        let expected = Tree::Program { 
            body: vec![
                Tree::VariableStatement { 
                    declarations: vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("x") }),
                            init: Box::new(Some(Tree::AssignmentExpression {
                                operator: String::from("="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                            })),
                        },
                    ],
                },
            ],
        };
        assert_tree(expected, "let x = y = 42;");
    }

    #[test]
    fn test_parse_chained_variable_statement_2() {
        let expected = Tree::Program { 
            body: vec![
                Tree::VariableStatement { 
                    declarations: vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("x") }),
                            init: Box::new(Some(Tree::AssignmentExpression {
                                operator: String::from("-="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                            })),
                        },
                    ],
                },
            ],
        };
        assert_tree(expected, "let x = y -= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_operator_variable_statement() {
        let expected = Error::Syntax(
            "Unexpected token ComplexAssignmentOperator, expected SimpleAssignmentOperator!".to_string()
        );
        assert_syntax_error(expected, "let x *= 42;");
    }

    #[test]
    fn test_parse_literal_eq_literal_variable_statement() {
        let expected = Error::Syntax(
            "Unexpected token Number, expected Identifier!".to_string()
        );
        assert_syntax_error(expected, "let 42 = 42;");
    }
}
