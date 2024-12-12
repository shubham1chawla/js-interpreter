use eatable::Eatable;
use identifier::IdentifierParsable;
use statements::{block::BlockStatementParsable, function::FunctionDeclarationParsable};

use super::*;

pub trait FunctionExpressionParsable {
    /**
     * FunctionExpression
     *  : 'function' OptIdentifier '(' OptFormalParameterList ')' BlockStatement
     *  ;
     */
    fn function_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl FunctionExpressionParsable for Parser {
    fn function_expression(&mut self) -> Result<Tree, SyntaxError> {
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
            params: Box::new(params),
            body: Box::new(body),
        })
    }
}

#[cfg(test)]
mod tests {
    use expressions::tests::{assert_syntax_error, assert_tree};

    use super::*;

    #[test]
    fn test_parse_simple_function_expression_1() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::Identifier { name: String::from("square") }),
                        right: Box::new(Tree::FunctionExpression {
                            identifier: Box::new(None),
                            params: Box::new(vec![
                                Tree::Identifier { name: String::from("x") },
                            ]),
                            body: Box::new(Tree::BlockStatement {
                                body: Box::new(vec![
                                    Tree::ReturnStatement {
                                        argument: Box::new(Some(Tree::BinaryExpression {
                                            operator: String::from("*"),
                                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                                            right: Box::new(Tree::Identifier { name: String::from("x") }),
                                        })),
                                    },
                                ]),
                            }),
                        }),
                    }),
                }
            ]),
        };
        assert_tree(expected, "square = function (x) { return x * x; };");
    }

    #[test]
    fn test_parse_simple_function_expression_2() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::FunctionExpression {
                            identifier: Box::new(Some(Tree::Identifier { name: String::from("y") })),
                            params: Box::new(vec![]),
                            body: Box::new(Tree::BlockStatement {
                                body: Box::new(vec![]),
                            }),
                        }),
                    }),
                }
            ]),
        };
        assert_tree(expected, "x = function y() {};");
    }

    #[test]
    fn test_parse_assigning_rvalue_function_expression() {
        let expected = SyntaxError {
            message: String::from("Invalid left-hand side in assignment expression, expected Identifier or MemberExpression!"),
        };
        assert_syntax_error(expected, "let x = function(){} = y;");
    }

    #[test]
    fn test_parse_invalid_function_expression() {
        let expected = SyntaxError {
            message: String::from("Unexpected token CircleBracketOpen, expected Identifier!"),
        };
        assert_syntax_error(expected, "function() {};");
    }
}
