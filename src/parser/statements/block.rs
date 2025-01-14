use crate::prelude::*;

use super::list::StatementListParsable;

pub trait BlockStatementParsable {
    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    fn block_statement(&mut self) -> Result<Tree>;
}

impl BlockStatementParsable for Parser {
    fn block_statement(&mut self) -> Result<Tree> {
        self.eat(TokenType::CurlyBracketOpen)?;
        let body = match self.lookahead.token_type {
            TokenType::CurlyBracketClose => vec![],
            _ => self.statement_list(TokenType::CurlyBracketClose)?,
        };
        self.eat(TokenType::CurlyBracketClose)?;
        Ok(Tree::BlockStatement { body })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::tests::*;

    #[test]
    fn test_parse_empty_block() {
        let expected = Tree::Program { 
            body: vec![
                Tree::BlockStatement { body: vec![] }
            ], 
        };
        assert_tree(expected, "{}");
    }

    #[test]
    fn test_parse_block_statements() {
        let expected = Tree::Program { 
            body: vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::BlockStatement { 
                    body: vec![
                        Tree::ExpressionStatement { 
                            expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                        }
                    ]
                },
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ], 
        };
        assert_tree(expected, "42; { //Commenting 42 -> 42;\n 'Hello'; } \"Hello\";");
    }

    #[test]
    fn test_parse_nested_block_statements() {
        let expected = Tree::Program { 
            body: vec![
                Tree::BlockStatement { 
                    body: vec![
                        Tree::BlockStatement { 
                            body: vec![
                                Tree::ExpressionStatement { 
                                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                                },
                                Tree::BlockStatement { 
                                    body: vec![
                                        Tree::ExpressionStatement { 
                                            expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                                        }
                                    ]
                                },
                            ] 
                        },
                        Tree::BlockStatement { 
                            body: vec![]
                        },
                    ]
                },
            ], 
        };
        assert_tree(expected, "{{ 42; { 'Hello'; } }{}}");
    }

    #[test]
    fn test_parse_invalid_block_statement() {
        let expected = Error::Syntax("Unexpected token Eof, expected Identifier!".to_string());
        assert_syntax_error(expected, "{");
    }
}
