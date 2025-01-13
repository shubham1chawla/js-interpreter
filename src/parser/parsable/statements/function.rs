use crate::prelude::*;

use super::block::BlockStatementParsable;
use super::eatable::Eatable;
use super::expression::ExpressionStatementParsable;
use super::identifier::IdentifierParsable;

pub trait FunctionDeclarationParsable {
    /**
     * FunctionDeclaration
     *  : 'function' Identifier '(' OptFormalParameterList ')' BlockStatement
     *  ;
     */
    fn function_declaration(&mut self) -> Result<Tree>;

    /**
     * FormalParameterList
     *  : Identifier
     *  | FormalParameterList ',' Identifier
     *  ;
     */
    fn formal_parameter_list(&mut self) -> Result<Vec<Tree>>;

    /**
     * ReturnStatement
     *  : 'return' OptExpression ';'
     *  ;
     */
    fn return_statement(&mut self) -> Result<Tree>;
}

impl FunctionDeclarationParsable for Parser {
    fn function_declaration(&mut self) -> Result<Tree> {
        self.eat(TokenType::FunctionKeyword)?;
        let identifier = self.identifier()?;

        self.eat(TokenType::CircleBracketOpen)?;

        // OptFormalParameterList
        let params = match self.lookahead.token_type {
            TokenType::CircleBracketClose => vec![],
            _ => self.formal_parameter_list()?,
        };

        self.eat(TokenType::CircleBracketClose)?;

        let body = self.block_statement()?;

        Ok(Tree::FunctionDeclaration {
            identifier: Box::new(identifier),
            params: Box::new(params),
            body: Box::new(body),
        })
    }
    
    fn formal_parameter_list(&mut self) -> Result<Vec<Tree>> {
        let mut params = vec![];

        while self.lookahead.token_type == TokenType::Identifier {
            params.push(self.identifier()?);

            // Consuming Commas
            if self.lookahead.token_type == TokenType::Comma {
                self.eat(TokenType::Comma)?;
            }
        }

        Ok(params)
    }

    fn return_statement(&mut self) -> Result<Tree> {
        self.eat(TokenType::ReturnKeyword)?;
        
        let argument = match self.lookahead.token_type {
            TokenType::SemiColon => None,
            _ => Some(self.expression()?),
        };
        self.eat(TokenType::SemiColon)?;
        
        Ok(Tree::ReturnStatement {
            argument: Box::new(argument),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

    #[test]
    fn test_parse_function_declaration_1() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::FunctionDeclaration {
                    identifier: Box::new(Tree::Identifier { name: String::from("hello") }),
                    params: Box::new(vec![]),
                    body: Box::new(Tree::BlockStatement { body: Box::new(vec![]) }),
                },
            ]),
        };
        assert_tree(expected, "function hello() {}");
    }

    #[test]
    fn test_parse_function_declaration_2() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::FunctionDeclaration {
                    identifier: Box::new(Tree::Identifier { name: String::from("multiply") }),
                    params: Box::new(vec![
                        Tree::Identifier { name: String::from("x") },
                        Tree::Identifier { name: String::from("y") },
                    ]),
                    body: Box::new(Tree::BlockStatement {
                        body: Box::new(vec![
                            Tree::ReturnStatement {
                                argument: Box::new(Some(Tree::BinaryExpression {
                                    operator: String::from("*"),
                                    left: Box::new(Tree::Identifier { name: String::from("x") }),
                                    right: Box::new(Tree::Identifier { name: String::from("y") }),
                                })),
                            },
                        ]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "function multiply(x, y) { return x * y; }");
    }

    #[test]
    fn test_parse_function_declaration_3() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::FunctionDeclaration {
                    identifier: Box::new(Tree::Identifier { name: String::from("test") }),
                    params: Box::new(vec![]),
                    body: Box::new(Tree::BlockStatement {
                        body: Box::new(vec![
                            Tree::VariableStatement {
                                declarations: Box::new(vec![
                                    Tree::VariableDeclaration {
                                        identifier: Box::new(Tree::Identifier { name: String::from("x") }),
                                        init: Box::new(Some(Tree::NumericLiteral { value: 10.0 })),
                                    },
                                ]),
                            },
                        ]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "function test() { let x = 10; }");
    }

    #[test]
    fn test_parse_function_declaration_4() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::FunctionDeclaration {
                    identifier: Box::new(Tree::Identifier { name: String::from("test") }),
                    params: Box::new(vec![]),
                    body: Box::new(Tree::BlockStatement {
                        body: Box::new(vec![
                            Tree::ReturnStatement {
                                argument: Box::new(None),
                            },
                        ]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "function test() { return; }");
    }

    #[test]
    fn test_parse_invalid_function_declaration_1() {
        let expected = Error::Syntax("Unexpected token SemiColon, expected CurlyBracketOpen!".to_string());
        assert_syntax_error(expected, "function why();");
    }
}
