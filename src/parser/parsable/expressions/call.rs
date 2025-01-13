use crate::prelude::*;

use super::assignment::AssignmentExpressionParsable;
use super::eatable::Eatable;
use super::member::MemberExpressionParsable;

pub trait CallExpressionParsable {
    /**
     * CallMemberExpression
     *  : MemberExpression
     *  | CallExpression
     *  ;
     */
    fn call_member_expression(&mut self) -> Result<Tree>;

    /**
     * CallExpression
     *  : Callee Arguments
     *  ;
     * 
     * Callee
     *  : MemberExpression
     *  | CallExpression
     *  ;
     */
    fn call_expression(&mut self, callee: Tree) -> Result<Tree>;

    /**
     * Arguments
     *  : '(' OptArgumentList ')'
     *  ;
     */
    fn arguments(&mut self) -> Result<Vec<Tree>>;

    /**
     * ArgumentList
     *  : AssignmentExpression
     *  | ArgumentList ',' AssignmentExpression
     *  ;
     */
    fn argument_list(&mut self) -> Result<Vec<Tree>>;
}

impl CallExpressionParsable for Parser {
    fn call_member_expression(&mut self) -> Result<Tree> {
        // Member part, might be part of a call
        let member = self.member_expression()?;

        // See if we have a call expression
        if self.lookahead.token_type == TokenType::CircleBracketOpen {
            return Ok(self.call_expression(member)?);
        }

        // Simple member expression
        Ok(member)
    }

    fn call_expression(&mut self, callee: Tree) -> Result<Tree> {
        let mut call_expression = Tree::CallExpression {
            callee: Box::new(callee),
            arguments: Box::new(self.arguments()?),
        };

        // Recursively checking if chained functions are called -> callback()();
        if self.lookahead.token_type == TokenType::CircleBracketOpen {
            call_expression = self.call_expression(call_expression)?;
        }

        Ok(call_expression)
    }

    fn arguments(&mut self) -> Result<Vec<Tree>> {
        self.eat(TokenType::CircleBracketOpen)?;

        let arguments = match self.lookahead.token_type {
            TokenType::CircleBracketClose => vec![],
            _ => self.argument_list()?,
        };

        self.eat(TokenType::CircleBracketClose)?;

        Ok(arguments)
    }

    fn argument_list(&mut self) -> Result<Vec<Tree>> {
        let mut arguments = vec![];

        // Consuming arguments until we hit the ')' token
        while self.lookahead.token_type != TokenType::CircleBracketClose {
            arguments.push(self.assignment_expression()?);

            // Consuming Commas
            if self.lookahead.token_type == TokenType::Comma {
                self.eat(TokenType::Comma)?;
            }
        }

        Ok(arguments)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

    #[test]
    fn test_parse_simple_call_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::CallExpression {
                        callee: Box::new(Tree::Identifier { name: String::from("foo") }),
                        arguments: Box::new(vec![
                            Tree::Identifier { name: String::from("x") },
                        ]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "foo(x);");
    }

    #[test]
    fn test_parse_chained_call_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::CallExpression {
                        callee: Box::new(Tree::CallExpression {
                            callee: Box::new(Tree::Identifier { name: String::from("foo") }),
                            arguments: Box::new(vec![
                                Tree::Identifier { name: String::from("x") },
                            ]),
                        }),
                        arguments: Box::new(vec![]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "foo(x)();");
    }

    #[test]
    fn test_parse_complex_call_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::CallExpression {
                        callee: Box::new(Tree::MemberExpression {
                            object: Box::new(Tree::Identifier { name: String::from("console") }),
                            property: Box::new(Tree::Identifier { name: String::from("log") }),
                            computed: false,
                        }),
                        arguments: Box::new(vec![
                            Tree::BinaryExpression {
                                operator: String::from(">"),
                                left: Box::new(Tree::Identifier { name: String::from("x") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                            },
                            Tree::AssignmentExpression {
                                operator: String::from("="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::BooleanLiteral { value: true }),
                            },
                        ]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "console.log(x > 42, y = true);");
    }
}
