use crate::prelude::*;

use super::eatable::Eatable;
use super::expression::ExpressionStatementParsable;
use super::list::StatementListParsable;
use super::variable::VariableStatementParsable;

pub trait IterationStatementParsable {
    /**
     * IterationStatement
     *  : WhileStatement
     *  | DoWhileStatement
     *  | ForStatement
     *  ;
     */
    fn iteration_statement(&mut self) -> Result<Tree>;

    /**
     * WhileStatement
     *  : 'while' '(' Expression ')' Statement
     *  ;
     */
    fn while_statement(&mut self) -> Result<Tree>;

    /**
     * DoWhileStatement
     *  : 'do' Statement '(' Expression ')' ';'
     *  ;
     */
    fn do_while_statement(&mut self) -> Result<Tree>;

    /**
     * ForStatement
     *  : 'for' '(' OptForStatementInit ';' OptExpression ';' OptExpression ')' Statement
     *  ;
     */
    fn for_statement(&mut self) -> Result<Tree>;

    /**
     * ForStatementInit
     *  : VariableStatementInit
     *  | Expression
     *  ;
     */
    fn for_statement_init(&mut self) -> Result<Tree>;
}

impl IterationStatementParsable for Parser {
    fn iteration_statement(&mut self) -> Result<Tree> {
        match self.lookahead.token_type {
            TokenType::DoKeyword => self.do_while_statement(),
            TokenType::ForKeyword => self.for_statement(),
            _ => self.while_statement()
        }
    }

    fn while_statement(&mut self) -> Result<Tree> {
        self.eat(TokenType::WhileKeyword)?;

        self.eat(TokenType::CircleBracketOpen)?;
        let expression = self.expression()?;
        self.eat(TokenType::CircleBracketClose)?;

        let body = self.statement()?;

        Ok(Tree::WhileStatement {
            test: Box::new(expression),
            body: Box::new(body),
        })
    }

    fn do_while_statement(&mut self) -> Result<Tree> {
        self.eat(TokenType::DoKeyword)?;

        let body = self.statement()?;
        self.eat(TokenType::WhileKeyword)?;

        self.eat(TokenType::CircleBracketOpen)?;
        let test = self.expression()?;
        self.eat(TokenType::CircleBracketClose)?;

        self.eat(TokenType::SemiColon)?;

        Ok(Tree::DoWhileStatement {
            body: Box::new(body),
            test: Box::new(test),
        })
    }

    fn for_statement(&mut self) -> Result<Tree> {
        self.eat(TokenType::ForKeyword)?;
        self.eat(TokenType::CircleBracketOpen)?;

        let init = match self.lookahead.token_type {
            TokenType::SemiColon => None,
            _ => Some(self.for_statement_init()?),
        };
        self.eat(TokenType::SemiColon)?;

        let test = match self.lookahead.token_type {
            TokenType::SemiColon => None,
            _ => Some(self.expression()?),
        };
        self.eat(TokenType::SemiColon)?;

        let update = match self.lookahead.token_type {
            TokenType::CircleBracketClose => None,
            _ => Some(self.expression()?),
        };
        self.eat(TokenType::CircleBracketClose)?;

        let body = self.statement()?;

        Ok(Tree::ForStatement {
            init: Box::new(init),
            test: Box::new(test),
            update: Box::new(update),
            body: Box::new(body),
        })
    }

    fn for_statement_init(&mut self) -> Result<Tree> {
        match self.lookahead.token_type {
            TokenType::LetKeyword => self.variable_statement_init(),
            _ => self.expression(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

    #[test]
    fn test_parse_while_statement_1() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::WhileStatement {
                    test: Box::new(Tree::BooleanLiteral { value: true }),
                    body: Box::new(Tree::EmptyStatement),
                },
            ]),
        };
        assert_tree(expected, "while (true);");
    }

    #[test]
    fn test_parse_while_statement_2() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::WhileStatement {
                    test: Box::new(Tree::BinaryExpression {
                        operator: String::from(">"),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }),
                    body: Box::new(Tree::BlockStatement {
                        body: Box::new(vec![
                            Tree::ExpressionStatement {
                                expression: Box::new(Tree::AssignmentExpression {
                                    operator: String::from("-="),
                                    left: Box::new(Tree::Identifier { name: String::from("x") }),
                                    right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                                }),
                            },
                        ]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "while (x > 42) { x -= 1; }");
    }

    #[test]
    fn test_parse_do_while_statement_1() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::DoWhileStatement {
                    body: Box::new(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("+="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                        }),
                    }),
                    test: Box::new(Tree::BinaryExpression {
                        operator: String::from("<"),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "do x += 1; while (x < 42);");
    }

    #[test]
    fn test_parse_for_statement_1() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ForStatement {
                    init: Box::new(None),
                    test: Box::new(None),
                    update: Box::new(None),
                    body: Box::new(Tree::EmptyStatement),
                },
            ]),
        };
        assert_tree(expected, "for (;;);");
    }

    #[test]
    fn test_parse_for_statement_2() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ForStatement {
                    init: Box::new(Some(Tree::VariableStatement {
                        declarations: Box::new(vec![
                            Tree::VariableDeclaration {
                                identifier: Box::new(Tree::Identifier { name: String::from("i") }),
                                init: Box::new(Some(Tree::NumericLiteral { value: 0.0 })),
                            },
                            Tree::VariableDeclaration {
                                identifier: Box::new(Tree::Identifier { name: String::from("x") }),
                                init: Box::new(Some(Tree::Identifier { name: String::from("y") })),
                            },
                        ]),
                    })),
                    test: Box::new(Some(Tree::BinaryExpression {
                        operator: String::from("<"),
                        left: Box::new(Tree::Identifier { name: String::from("i") }),
                        right: Box::new(Tree::NumericLiteral { value: 10.0 }),
                    })),
                    update: Box::new(Some(Tree::AssignmentExpression {
                        operator: String::from("+="),
                        left: Box::new(Tree::Identifier { name: String::from("i") }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    })),
                    body: Box::new(Tree::BlockStatement {
                        body: Box::new(vec![]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "for (let i=0, x=y; i<10; i+=1) {}");
    }

    #[test]
    fn test_parse_for_statement_3() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ForStatement {
                    init: Box::new(Some(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::NumericLiteral { value: 2.0 }),
                    })),
                    test: Box::new(None),
                    update: Box::new(None),
                    body: Box::new(Tree::EmptyStatement),
                },
            ]),
        };
        assert_tree(expected, "for (x = 2;;);");
    }
}
