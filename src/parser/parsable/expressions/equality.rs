use eatable::Eatable;
use relational::RelationalExpressionParsable;

use super::*;

pub trait EqualityExpressionParsable {
    /**
     * EqualityExpression
     *  : RelationalExpression EQUALITY_OPERATOR EqualityExpression
     *  | RelationalExpression
     *  ;
     * 
     * NOTE: Since RelationalExpression has higher presidence over EqualityExpression
     * the left and right sub-tree of EqualityExpression looks for a RelationalExpression.
     */
    fn equality_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl EqualityExpressionParsable for Parser {
    fn equality_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.relational_expression()?;

        while self.lookahead.token_type == TokenType::EqualityOperator {
            // Operator: ==, !=
            let operator = self.eat(TokenType::EqualityOperator)?.value;

            // Extracting the right literal
            let right = self.relational_expression()?;
            
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
    use expressions::tests::assert_tree;

    use super::*;

    #[test]
    fn test_parse_simple_equality_expression_1() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::BinaryExpression {
                        operator: String::from("=="),
                        left: Box::new(Tree::BinaryExpression {
                            operator: String::from(">"),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 0.0 }),
                        }),
                        right: Box::new(Tree::BooleanLiteral { value: true, }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "x > 0 == true;");
    }

    #[test]
    fn test_parse_simple_equality_expression_2() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::BinaryExpression {
                        operator: String::from("!="),
                        left: Box::new(Tree::BinaryExpression {
                            operator: String::from("<="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 0.0 }),
                        }),
                        right: Box::new(Tree::BooleanLiteral { value: false, }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "x <= 0 != false;");
    }

    #[test]
    fn test_parse_simple_equality_expression_3() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::BinaryExpression {
                        operator: String::from("!="),
                        left: Box::new(Tree::BooleanLiteral { value: true, }),
                        right: Box::new(Tree::BooleanLiteral { value: false, }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "true != false;");
    }
}