use crate::prelude::*;

use super::eatable::Eatable;
use super::logical::LogicalExpressionParsable;

pub trait AssignmentExpressionParsable {
    /**
     * AssignmentExpression
     *  : LogicalOrExpression
     *  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn assignment_expression(&mut self) -> Result<Tree>;

    /**
     * Whether the token is an assignment operator.
     */
    fn is_assignment_operator(&self) -> bool;

    /**
     * Extra check whether it's valid assignment target.
     */
    fn check_valid_assignment_target(&mut self, node: Tree) -> Result<Tree>;

    /**
     * AssignmentOperator
     *  : SIMPLE_ASSIGNMENT_OPERATOR
     *  | COMPLEX_ASSIGNMENT_OPERATOR
     *  ;
     */
    fn assignment_operator(&mut self) -> Result<Token>;
}

impl AssignmentExpressionParsable for Parser {
    fn assignment_expression(&mut self) -> Result<Tree> {
        let mut left = self.logical_or_expression()?;

        // Checking if the lookahead token is not of assignment type
        if !self.is_assignment_operator() {
            return Ok(left);
        }

        // Consuming assignment operator
        let operator = self.assignment_operator()?.value;

        // Checking if the left hand side expression is valid, aka an identifier
        left = self.check_valid_assignment_target(left)?;

        // Right-recursing to create the remaining expression
        let right = self.assignment_expression()?;

        Ok(Tree::AssignmentExpression { 
            operator, 
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn is_assignment_operator(&self) -> bool {
        match self.lookahead.token_type {
            TokenType::SimpleAssignmentOperator | TokenType::ComplexAssignmentOperator => true,
            _ => false,
        }
    }

    fn check_valid_assignment_target(&mut self, node: Tree) -> Result<Tree> {
        match node {
            Tree::Identifier {..} | Tree::MemberExpression {..} => Ok(node),
            _ => Err(Error::Syntax(
                "Invalid left-hand side in assignment expression, expected Identifier or MemberExpression!".to_string()
            ))
        }
    }

    fn assignment_operator(&mut self) -> Result<Token> {
        match self.lookahead.token_type {
            TokenType::SimpleAssignmentOperator => self.eat(TokenType::SimpleAssignmentOperator),
            _ => self.eat(TokenType::ComplexAssignmentOperator),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

    #[test]
    fn test_parse_simple_assignment_expression_1() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num = 42;");
    }

    #[test]
    fn test_parse_simple_assignment_expression_2() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("str") }), 
                        right: Box::new(Tree::StringLiteral { value: String::from("Hello, World!") }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "str = 'Hello, World!';");
    }

    #[test]
    fn test_parse_simple_assignment_expression_3() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("xyz") }), 
                        right: Box::new(Tree::BinaryExpression { 
                            operator: String::from("+"), 
                            left: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                            right: Box::new(Tree::NumericLiteral { value: 3.0 }),
                        }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "xyz = 2 + 3;");
    }

    #[test]
    fn test_parse_chained_assignment_expression() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("x") }), 
                        right: Box::new(Tree::AssignmentExpression { 
                            operator: String::from("="), 
                            left: Box::new(Tree::Identifier { name: String::from("y") }), 
                            right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                        }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "x = y = 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_1() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("+="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num += 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_2() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("-="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num -= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_3() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("*="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num *= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_4() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("/="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num /= 42;");
    }

    #[test]
    fn test_parse_invalid_assignment_expression() {
        let expected = Error::Syntax(
            "Invalid left-hand side in assignment expression, expected Identifier or MemberExpression!".to_string()
        );
        assert_syntax_error(expected, "42 = 42;");
    }

    #[test]
    fn test_parse_presidence_assignment_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::VariableStatement {
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("isSomething") }),
                            init: Box::new(Some(Tree::BinaryExpression {
                                operator: String::from("=="),
                                left: Box::new(Tree::BinaryExpression {
                                    operator: String::from("<"),
                                    left: Box::new(Tree::NumericLiteral { value: 50.0 }),
                                    right: Box::new(Tree::BinaryExpression {
                                        operator: String::from("+"),
                                        left: Box::new(Tree::Identifier { name: String::from("value") }),
                                        right: Box::new(Tree::BinaryExpression {
                                            operator: String::from("*"),
                                            left: Box::new(Tree::NumericLiteral { value: 5.0 }),
                                            right: Box::new(Tree::NumericLiteral { value: 2.0 }),
                                        }),
                                    }),
                                }),
                                right: Box::new(Tree::BooleanLiteral { value: true }),
                            })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let isSomething = 50 < value + 5 * 2 == true;");
    }
}