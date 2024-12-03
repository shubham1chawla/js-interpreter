use eatable::Eatable;
use equality::EqualityExpressionParsable;

use super::*;

pub trait LogicalExpressionParsable {
    /**
     * LogicalOrExpression
     *  : LogicalAndExpression
     *  | LogicalAndExpression '||' LogicalAndExpression
     *  ; 
     * 
     * NOTE: Since LogicalAndExpression has higher presidence over LogicalOrExpression
     * the left and right sub-tree of LogicalOrExpression looks for a LogicalAndExpression.
     */
    fn logical_or_expression(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * LogicalAndExpression
     *  : EqualityExpression
     *  | EqualityExpression '&&' EqualityExpression
     *  ;
     * 
     * NOTE: Since EqualityExpression has higher presidence over LogicalAndExpression
     * the left and right sub-tree of LogicalAndExpression looks for a EqualityExpression.
     */
    fn logical_and_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl LogicalExpressionParsable for Parser {
    fn logical_or_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.logical_and_expression()?;

        while self.lookahead.token_type == TokenType::LogicalOrOperator {
            // Operator: ||
            let operator = self.eat(TokenType::LogicalOrOperator)?.value;

            // Extracting the right literal
            let right = self.logical_and_expression()?;
            
            // Enforcing left associativity
            left = Tree::LogicalExpression { 
                operator, 
                left: Box::new(left), 
                right: Box::new(right), 
            };
        }
        Ok(left)
    }

    fn logical_and_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.equality_expression()?;

        while self.lookahead.token_type == TokenType::LogicalAndOperator {
            // Operator: &&
            let operator = self.eat(TokenType::LogicalAndOperator)?.value;

            // Extracting the right literal
            let right = self.equality_expression()?;
            
            // Enforcing left associativity
            left = Tree::LogicalExpression { 
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
    use expressions::tests::{assert_syntax_error, assert_tree};

    use super::*;

    #[test]
    fn test_parse_simple_and_logical_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::LogicalExpression {
                        operator: String::from("&&"),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::Identifier { name: String::from("y") }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "x && y;");
    }

    #[test]
    fn test_parse_simple_or_logical_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::LogicalExpression {
                        operator: String::from("||"),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::Identifier { name: String::from("y") }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "x || y;");
    }

    #[test]
    fn test_parse_complex_logical_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::LogicalExpression {
                        operator: String::from("||"),
                        left: Box::new(Tree::BinaryExpression {
                            operator: String::from(">"),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 0.0 }),
                        }),
                        right: Box::new(Tree::LogicalExpression {
                            operator: String::from("&&"),
                            left: Box::new(Tree::BinaryExpression {
                                operator: String::from("<"),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 0.0 }),
                            }),
                            right: Box::new(Tree::BinaryExpression {
                                operator: String::from("=="),
                                left: Box::new(Tree::Identifier { name: String::from("z") }),
                                right: Box::new(Tree::NullLiteral),
                            }),
                        }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "x > 0 || y < 0 && z == null;");
    }

    #[test]
    fn test_parse_invalid_and_logical_expression() {
        let expected = SyntaxError {
            message: String::from("Unexpected token: &"),
        };
        assert_syntax_error(expected, "x & y;");
    }

    #[test]
    fn test_parse_invalid_or_logical_expression() {
        let expected = SyntaxError {
            message: String::from("Unexpected token: |"),
        };
        assert_syntax_error(expected, "x | y;");
    }
}
