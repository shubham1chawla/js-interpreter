use crate::prelude::*;

use super::additive::AdditiveExpressionParsable;

pub trait RelationalExpressionParsable {
    /**
     * RelationalExpression
     *  : AdditiveExpression
     *  | AdditiveExpression RELATIONAL_OPERATOR RelationalExpression
     *  ;
     * 
     * NOTE: Since AdditiveExpression has higher presidence over RelationalExpression
     * the left and right sub-tree of RelationalExpression looks for a AdditiveExpression.
     */
    fn relational_expression(&mut self) -> Result<Tree>;
}

impl RelationalExpressionParsable for Parser {
    fn relational_expression(&mut self) -> Result<Tree> {
        let mut left = self.additive_expression()?;

        while self.lookahead.token_type == TokenType::RelationalOperator {
            // Operator: <, >, <=, >=
            let operator = self.eat(TokenType::RelationalOperator)?.value;

            // Extracting the right literal
            let right = self.additive_expression()?;
            
            // Enforcing left associativity
            left = Tree::BinaryExpression { 
                operator, 
                left: Box::new(left), 
                right: Box::new(right), 
            };
        }
        Ok(left)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::tests::*;

    #[test]
    fn test_parse_simple_relational_expression() {
        let expected = Tree::Program {
            body: vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::BinaryExpression {
                        operator: String::from(">="),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::NumericLiteral { value: 42.0, }),
                    }),
                },
            ],
        };
        assert_tree(expected, "x >= 42;");
    }

    #[test]
    fn test_parse_complex_relational_expression() {
        let expected = Tree::Program {
            body: vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::Identifier { name: String::from("y") }),
                        right: Box::new(Tree::BinaryExpression {
                            operator: String::from(">"),
                            left: Box::new(Tree::BinaryExpression {
                                operator: String::from("*"),
                                left: Box::new(Tree::BinaryExpression {
                                    operator: String::from("+"),
                                    left: Box::new(Tree::Identifier { name: String::from("x") }),
                                    right: Box::new(Tree::NumericLiteral { value: 10.0 }),
                                }),
                                right: Box::new(Tree::NumericLiteral { value: 3.0 }),
                            }),
                            right: Box::new(Tree::NumericLiteral { value: 100.0 }),
                        }),
                    }),
                },
            ],
        };
        assert_tree(expected, "y = (x + 10) * 3 > 100;");
    }

    #[test]
    fn test_parse_relational_expression_if_statement() {
        let expected = Tree::Program {
            body: vec![
                Tree::IfStatement {
                    test: Box::new(Tree::BinaryExpression {
                        operator: String::from("<"),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::NumericLiteral { value: 42.0, }),
                    }),
                    consequent: Box::new(Tree::BlockStatement { body: vec![] }),
                    alternate: Box::new(None),
                },
            ],
        };
        assert_tree(expected, "if (x < 42) {}");
    }
}