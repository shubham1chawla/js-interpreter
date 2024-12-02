use primary::PrimaryExpressionParsable;

use super::*;

pub trait MultiplicativeExpressionParsable {
    /**
     * MultiplicativeExpression
     *  : PrimaryExpression
     *  | MultiplicativeExpression MULTIPLICATIVE_OPERATOR PrimaryExpression
     *  ;
     * 
     * NOTE: Since PrimaryExpression has higher presidence over MultiplicativeExpression
     * the left and right sub-tree of MultiplicativeExpression looks for a PrimaryExpression.
     */
    fn multiplicative_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl MultiplicativeExpressionParsable for Parser {
    fn multiplicative_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.primary_expression()?;

        while self.lookahead.token_type == TokenType::MultiplicativeOperator {
            // Operator: *, /
            let operator = self.eat(TokenType::MultiplicativeOperator)?.value;

            // Extracting the right literal
            let right = self.primary_expression()?;
            
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
    use parsable::tests::assert_tree;

    use super::*;

    #[test]
    fn test_parse_multiplicative_binary_expressions() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::BinaryExpression { 
                        operator: String::from("*"), 
                        left: Box::new(Tree::BinaryExpression { 
                            operator: String::from("/"), 
                            left: Box::new(Tree::NumericLiteral { value: 3.0 }),
                            right: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                        }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    }),
                }
            ]), 
        };
        assert_tree(expected, "3 / 2 * 1;");
    }

    #[test]
    fn test_parse_multiplicative_additive_binary_expressions() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::BinaryExpression { 
                        operator: String::from("+"), 
                        left: Box::new(Tree::NumericLiteral { value: 3.0 }),
                        right: Box::new(Tree::BinaryExpression { 
                            operator: String::from("/"), 
                            left: Box::new(Tree::NumericLiteral { value: 2.0 }),
                            right: Box::new(Tree::NumericLiteral { value: 1.0 }), 
                        })
                    }),
                }
            ]), 
        };
        assert_tree(expected, "3 + 2 / 1;");
    }
}