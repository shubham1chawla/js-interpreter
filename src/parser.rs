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
    lookahead: Option<Token>,
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
        let statement_list = self.statement_list()?;
        return Ok(Tree::Program { body: Box::new(statement_list) });
    }

    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self) -> Result<Vec<Tree>, SyntaxError> {
        let mut statement_list = vec![];
        
        while self.lookahead.is_some() {
            statement_list.push(self.statement()?);
        }

        Ok(statement_list)
    }

    /**
     * Statement
     *  : ExpressionStatement
     *  ;
     */
    fn statement(&mut self) -> Result<Tree, SyntaxError> {
        self.expression_statement()
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
        let lookahead = self.lookahead.clone().unwrap();
        match lookahead.token_type {
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
        let lookahead = self.lookahead.clone();
        match lookahead {
            None => Err(SyntaxError {
                message: format(format_args!("Unexpected EOF, expected {:?}!", token_type)),
            }),
            Some(token) => {
                if token.token_type != token_type {
                    return Err(SyntaxError {
                        message: format(format_args!("Unexpected token {:?}, expected {:?}!", token.token_type, token_type)),
                    });
                }
                
                // Advance to the next token.
                self.lookahead = self.tokenizer.get_next_token()?;
                Ok(token)
            }
        }
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

}
