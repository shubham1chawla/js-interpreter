use eatable::Eatable;
use multiplicative::MultiplicativeExpressionParsable;

use super::*;

pub trait AdditiveExpressionParsable {
    /**
     * AdditiveExpression
     *  : MultiplicativeExpression
     *  | AdditiveExpression ADDITIVE_OPERATOR MultiplicativeExpression
     *  ;
     * 
     * NOTE: Since MultiplicativeExpression has higher presidence over AdditiveExpression
     * the left and right sub-tree of AdditiveExpression looks for a MultiplicativeExpression.
     */
    fn additive_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl AdditiveExpressionParsable for Parser {
    fn additive_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.multiplicative_expression()?;

        while self.lookahead.token_type == TokenType::AdditiveOperator {
            // Operator: +, -
            let operator = self.eat(TokenType::AdditiveOperator)?.value;

            // Extracting the right literal
            let right = self.multiplicative_expression()?;
            
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
    fn test_parse_additive_binary_expressions() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::BinaryExpression { 
                        operator: String::from("+"), 
                        left: Box::new(Tree::BinaryExpression { 
                            operator: String::from("-"), 
                            left: Box::new(Tree::NumericLiteral { value: 3.0 }),
                            right: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                        }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    }),
                }
            ]), 
        };
        assert_tree(expected, "3 - 2 + 1;");
    }
}