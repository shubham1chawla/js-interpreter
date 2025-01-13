use crate::prelude::*;

use super::eatable::Eatable;
use super::expression::ExpressionStatementParsable;
use super::list::StatementListParsable;

pub trait IfStatementParsable {
    /**
     * IfStatement
     *  : 'if' '(' Expression ')' Statement
     *  | 'if' '(' Expression ')' Statement 'else' Statement
     *  ;
     */
    fn if_statement(&mut self) -> Result<Tree>;
}

impl IfStatementParsable for Parser {
    fn if_statement(&mut self) -> Result<Tree> {
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

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
