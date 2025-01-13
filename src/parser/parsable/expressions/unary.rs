use crate::prelude::*;

use super::eatable::Eatable;
use super::lhs::LeftHandSideExpressionParsable;

pub trait UnaryExpressionParsable {
    /**
     * UnaryExpression
     *  : LeftHandSideExpression
     *  | ADDITIVE_OPERATOR UnaryExpression
     *  | LOGICAL_NOT UnaryExpression
     *  ;
     */
    fn unary_expression(&mut self) -> Result<Tree>;
}

impl UnaryExpressionParsable for Parser {
    fn unary_expression(&mut self) -> Result<Tree> {
        let operator = match self.lookahead.token_type {
            TokenType::AdditiveOperator => Some(self.eat(TokenType::AdditiveOperator)?),
            TokenType::LogicalNotOperator => Some(self.eat(TokenType::LogicalNotOperator)?),
            _ => None,
        };
        match operator {
            Some(token) => Ok(Tree::UnaryExpression {
                operator: token.value,
                argument: Box::new(self.unary_expression()?),
            }),
            None => self.left_hand_side_expression(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

    #[test]
    fn test_parse_simple_unary_expression_1() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::UnaryExpression {
                        operator: String::from("!"),
                        argument: Box::new(Tree::Identifier { name: String::from("x") }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "!x;");
    }

    #[test]
    fn test_parse_simple_unary_expression_2() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::UnaryExpression {
                        operator: String::from("+"),
                        argument: Box::new(Tree::Identifier { name: String::from("x") }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "+x;");
    }

    #[test]
    fn test_parse_simple_unary_expression_3() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::UnaryExpression {
                        operator: String::from("-"),
                        argument: Box::new(Tree::Identifier { name: String::from("x") }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "-x;");
    }

    #[test]
    fn test_parse_complex_unary_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::Identifier { name: String::from("y") }),
                        right: Box::new(Tree::LogicalExpression {
                            operator: String::from("&&"),
                            left: Box::new(Tree::UnaryExpression {
                                operator: String::from("!"),
                                argument: Box::new(Tree::LogicalExpression {
                                    operator: String::from("||"),
                                    left: Box::new(Tree::BinaryExpression {
                                        operator: String::from(">"),
                                        left: Box::new(Tree::Identifier { name: String::from("a") }),
                                        right: Box::new(Tree::NumericLiteral { value: 20.0 }),
                                    }),
                                    right: Box::new(Tree::BinaryExpression {
                                        operator: String::from("!="),
                                        left: Box::new(Tree::Identifier { name: String::from("b") }),
                                        right: Box::new(Tree::NullLiteral),
                                    }),
                                }),
                            }),
                            right: Box::new(Tree::BinaryExpression {
                                operator: String::from(">"),
                                left: Box::new(Tree::Identifier { name: String::from("c") }),
                                right: Box::new(Tree::NumericLiteral { value: 0.0 }),
                            }),
                        }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "y = !(a > 20 || b != null) && c > 0;");
    }
}
