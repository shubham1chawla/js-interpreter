use eatable::Eatable;
use statements::expression::ExpressionStatementParsable;

use super::*;

pub trait ParanthesizedExpressionParsable {
    /**
     * ParanthesizedExpression
     *  : '(' Expression ')'
     *  ;
     */
    fn paranthesized_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl ParanthesizedExpressionParsable for Parser {
    fn paranthesized_expression(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::CircleBracketOpen)?;
        let expression = self.expression()?;
        self.eat(TokenType::CircleBracketClose)?;
        Ok(expression)
    }
}

#[cfg(test)]
mod tests {
    use expressions::tests::assert_tree;

    use super::*;

    #[test]
    fn test_parse_paranthesized_binary_expressions() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::BinaryExpression { 
                        operator: String::from("/"), 
                        left: Box::new(Tree::BinaryExpression { 
                            operator: String::from("+"), 
                            left: Box::new(Tree::NumericLiteral { value: 3.0 }),
                            right: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                        }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    }),
                }
            ]), 
        };
        assert_tree(expected, "(3 + 2) / 1;");
    }
}
