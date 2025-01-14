use crate::prelude::*;

use super::statements::expression::ExpressionStatementParsable;

pub trait ParanthesizedExpressionParsable {
    /**
     * ParanthesizedExpression
     *  : '(' Expression ')'
     *  ;
     */
    fn paranthesized_expression(&mut self) -> Result<Tree>;
}

impl ParanthesizedExpressionParsable for Parser {
    fn paranthesized_expression(&mut self) -> Result<Tree> {
        self.eat(TokenType::CircleBracketOpen)?;
        let expression = self.expression()?;
        self.eat(TokenType::CircleBracketClose)?;
        Ok(expression)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::tests::*;

    #[test]
    fn test_parse_paranthesized_binary_expressions() {
        let expected = Tree::Program { 
            body: vec![
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
            ], 
        };
        assert_tree(expected, "(3 + 2) / 1;");
    }
}
