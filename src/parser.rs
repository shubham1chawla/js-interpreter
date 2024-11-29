use std::fmt::format;

use tree::Tree;
use tokenizer::{SyntaxError, Token, TokenType, Tokenizer};

mod tree;
mod tokenizer;

/**
 * Letter parser: recursive decent parser implementation
 */
pub struct Parser {
    tokenizer: Tokenizer,
    lookahead: Token,
}

impl Parser {

    /**
     * Creates a new parser instance with code content as string.
     */
    pub fn new(content_string: String) -> Result<Self, SyntaxError> {
        let mut tokenizer = Tokenizer::new(content_string);

        // Prime the tokenizer to obtain the first token
        // which is our lookahead. The lookahead is used
        // for predictive parsing.
        let lookahead = tokenizer.get_next_token()?;

        Ok(Self {
            tokenizer,
            lookahead,
        })
    }

    /**
     * Parses a string into an AST.
     */
    pub fn parse(&mut self) -> Result<Tree, SyntaxError> {
        self.program()
    }

    /**
     * This function is the main entry point of the parser.
     * 
     * Program
     *  : StatementList
     *  ;
     */
    fn program(&mut self) -> Result<Tree, SyntaxError> {
        let statement_list = self.statement_list(TokenType::EOF)?;
        return Ok(Tree::Program { body: Box::new(statement_list) });
    }

    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_lookahead_type: TokenType) -> Result<Vec<Tree>, SyntaxError> {
        let mut statement_list = vec![];

        while self.lookahead.token_type != stop_lookahead_type {
            statement_list.push(self.statement()?);
        }

        Ok(statement_list)
    }

    /**
     * Statement
     *  : EmptyStatement
     *  | BlockStatement
     *  | ExpressionStatement
     *  ;
     */
    fn statement(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::SemiColon => self.empty_statement(),
            TokenType::CurlyBracketOpen => self.block_statement(),
            _ => self.expression_statement(),
        }
    }

    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    fn empty_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::EmptyStatement)
    }

    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    fn block_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::CurlyBracketOpen)?;
        let body = match self.lookahead.token_type {
            TokenType::CurlyBracketClose => vec![],
            _ => self.statement_list(TokenType::CurlyBracketClose)?,
        };
        self.eat(TokenType::CurlyBracketClose)?;
        Ok(Tree::BlockStatement { body: Box::new(body) })
    }

    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self) -> Result<Tree, SyntaxError> {
        let expression = self.expression()?;
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::ExpressionStatement { expression: Box::new(expression) })
    }

    /**
     * Expression
     *  : Literal
     *  ;
     */
    fn expression(&mut self) -> Result<Tree, SyntaxError> {
        self.literal()
    }

    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::Number => self.numeric_literal(),
            _ => self.string_literal(),
        }
    }

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> Result<Tree, SyntaxError> {
        let token = self.eat(TokenType::Number)?;

        let parsed = token.value.parse().expect("Expected a numeric value!");
        return Ok(Tree::NumericLiteral { value: parsed })
    }

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> Result<Tree, SyntaxError> {
        let token = self.eat(TokenType::String)?;

        // Removing quotes from start and end
        let value = String::from(&token.value[1..(token.value.len()-1)]);
        return Ok(Tree::StringLiteral { value })
    }

    /**
     * Eats a token and advances the lookahead token.
     * Throws a Syntax error if lookahead doesn't match supplied token.
     */
    fn eat(&mut self, token_type: TokenType) -> Result<Token, SyntaxError> {
        if self.lookahead.token_type != token_type {
            return Err(SyntaxError {
                message: format(format_args!("Unexpected token {:?}, expected {:?}!", self.lookahead.token_type, token_type)),
            });
        }
        
        // Advance to the next token.
        let token = self.lookahead.clone();
        self.lookahead = self.tokenizer.get_next_token()?;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tree(expected: Tree, content_string: &str) {
        let parser_result = Parser::new(content_string.to_owned());
        assert!(parser_result.is_ok());
        let mut parser = parser_result.unwrap();
        let tree_result = parser.parse();
        assert!(tree_result.is_ok());
        assert_eq!(expected, tree_result.unwrap());
    }

    // ----- TESTS FOR COMMENTS ----- //

    #[test]
    fn test_parse_single_line_comments() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                }
            ]), 
        };
        assert_tree(expected, "// Comment \n 42;");
    }

    #[test]
    fn test_parse_multi_line_comments() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]), 
        };
        let content_string = "\
        /* Multi-line comment \n\
        * Hello
        */
        \"Hello\";";
        assert_tree(expected, content_string);
    }

    // ----- TESTS FOR LITERALS ----- //

    #[test]
    fn test_parse_literal_numeric() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                }
            ]), 
        };
        assert_tree(expected, "42;");
    }

    #[test]
    fn test_parse_literal_string() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]), 
        };
        assert_tree(expected, "\"Hello\";");
    }

    // ----- TESTS FOR WHITESPACES ----- //

    #[test]
    fn test_parse_literal_numeric_with_whitespaces() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                }
            ]), 
        };
        assert_tree(expected, "    42;");
    }

    #[test]
    fn test_parse_literal_string_with_whitespaces() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "  Hello, World!  ".to_owned() } ),
                }
            ]), 
        };
        assert_tree(expected, "  \"  Hello, World!  \";  ");
    }

    // ----- TESTS FOR EMPTY FILE ----- //

    #[test]
    fn test_parse_empty_content() {
        let expected = Tree::Program { body: Box::new(vec![]) };
        assert_tree(expected, "");
    }

    // ----- TESTS FOR STATEMENT LIST ----- //

    #[test]
    fn test_parse_statement_list() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]),
        };
        assert_tree(expected, "42;\"Hello\";");
    }

    // ----- TESTS FOR BLOCK STATEMENT ----- //

    #[test]
    fn test_parse_empty_block() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::BlockStatement { body: Box::new(vec![]) }
            ]), 
        };
        assert_tree(expected, "{}");
    }

    #[test]
    fn test_parse_block_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::BlockStatement { 
                    body: Box::new(vec![
                        Tree::ExpressionStatement { 
                            expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                        }
                    ]) 
                },
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]), 
        };
        assert_tree(expected, "42; { //Commenting 42 -> 42;\n 'Hello'; } \"Hello\";");
    }

    #[test]
    fn test_parse_nested_block_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::BlockStatement { 
                    body: Box::new(vec![
                        Tree::BlockStatement { 
                            body: Box::new(vec![
                                Tree::ExpressionStatement { 
                                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                                },
                                Tree::BlockStatement { 
                                    body: Box::new(vec![
                                        Tree::ExpressionStatement { 
                                            expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                                        }
                                    ]) 
                                },
                            ]) 
                        },
                        Tree::BlockStatement { 
                            body: Box::new(vec![]) 
                        },
                    ]) 
                },
            ]), 
        };
        assert_tree(expected, "{{ 42; { 'Hello'; } }{}}");
    }

    // ----- TESTS FOR EMPTY STATEMENT ----- //

    #[test]
    fn test_parse_simple_empty_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::EmptyStatement,
            ]),
        };
        assert_tree(expected, ";");
    }

    #[test]
    fn test_parse_empty_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::EmptyStatement,
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::EmptyStatement,
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]),
        };
        assert_tree(expected, ";\n42;\n;\n'Hello';");
    }

}
