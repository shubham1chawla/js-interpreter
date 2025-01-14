use crate::prelude::*;

use super::identifier::IdentifierParsable;
use super::statements::block::BlockStatementParsable;
use super::statements::function::FunctionDeclarationParsable;

pub trait FunctionExpressionParsable {
    /**
     * FunctionExpression
     *  : 'function' OptIdentifier '(' OptFormalParameterList ')' BlockStatement
     *  ;
     */
    fn function_expression(&mut self) -> Result<Tree>;
}

impl FunctionExpressionParsable for Parser {
    fn function_expression(&mut self) -> Result<Tree> {
        self.eat(TokenType::FunctionKeyword)?;

        let identifier = match self.lookahead.token_type {
            TokenType::CircleBracketOpen => None,
            _ => Some(self.identifier()?),
        };

        self.eat(TokenType::CircleBracketOpen)?;
        
        // OptFormalParameterList
        let params = self.formal_parameter_list()?;

        self.eat(TokenType::CircleBracketClose)?;

        let body = self.block_statement()?;

        Ok(Tree::FunctionExpression {
            identifier: Box::new(identifier),
            params,
            body: Box::new(body),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::tests::*;

    #[test]
    fn test_parse_simple_function_expression_1() {
        let expected = Tree::Program {
            body: vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::Identifier { name: String::from("square") }),
                        right: Box::new(Tree::FunctionExpression {
                            identifier: Box::new(None),
                            params: vec![
                                Tree::Identifier { name: String::from("x") },
                            ],
                            body: Box::new(Tree::BlockStatement {
                                body: vec![
                                    Tree::ReturnStatement {
                                        argument: Box::new(Some(Tree::BinaryExpression {
                                            operator: String::from("*"),
                                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                                            right: Box::new(Tree::Identifier { name: String::from("x") }),
                                        })),
                                    },
                                ],
                            }),
                        }),
                    }),
                }
            ],
        };
        assert_tree(expected, "square = function (x) { return x * x; };");
    }

    #[test]
    fn test_parse_simple_function_expression_2() {
        let expected = Tree::Program {
            body: vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::FunctionExpression {
                            identifier: Box::new(Some(Tree::Identifier { name: String::from("y") })),
                            params: vec![],
                            body: Box::new(Tree::BlockStatement {
                                body: vec![],
                            }),
                        }),
                    }),
                }
            ],
        };
        assert_tree(expected, "x = function y() {};");
    }

    #[test]
    fn test_parse_assigning_rvalue_function_expression() {
        let expected = Error::Syntax(
            "Invalid left-hand side in assignment expression, expected Identifier or MemberExpression!".to_string()
        );
        assert_syntax_error(expected, "let x = function(){} = y;");
    }

    #[test]
    fn test_parse_invalid_function_expression() {
        let expected = Error::Syntax(
            "Unexpected token CircleBracketOpen, expected Identifier!".to_string()
        );
        assert_syntax_error(expected, "function() {};");
    }
}
