use eatable::Eatable;
use expression::ExpressionStatementParsable;
use list::StatementListParsable;

use super::*;

pub trait IterationStatementParsable {
    /**
     * IterationStatement
     *  : WhileStatement
     *  | DoWhileStatement
     *  | ForStatement
     *  ;
     */
    fn iteration_statement(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * WhileStatement
     *  : 'while' '(' Expression ')' Statement
     *  ;
     */
    fn while_statement(&mut self) -> Result<Tree, SyntaxError>;
}

impl IterationStatementParsable for Parser {
    fn iteration_statement(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            _ => self.while_statement()
        }
    }

    fn while_statement(&mut self) -> Result<Tree, SyntaxError> {
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
}

#[cfg(test)]
mod tests {
    use statements::tests::assert_tree;

    use super::*;

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
}
