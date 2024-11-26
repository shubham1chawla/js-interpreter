use std::fmt::format;

use tree::{Tree, TreeNodeType};
use tokenizer::{Token, TokenType, Tokenizer};

mod tree;
mod tokenizer;

/**
 * Letter parser: recursive decent parser implementation
 */
pub struct Parser {
    tokenizer: Box<Tokenizer>,
    lookahead: Option<Token>,
}

impl Parser {

    /**
     * Creates a new parser instance with code content as string.
     */
    pub fn new(content_string: String) -> Self {
        let mut tokenizer = Tokenizer::new(content_string);

        // Prime the tokenizer to obtain the first token
        // which is our lookahead. The lookahead is used
        // for predictive parsing.
        let lookahead = tokenizer.get_next_token();

        Self {
            tokenizer: Box::new(tokenizer),
            lookahead: lookahead.to_owned(),
        }
    }

    /**
     * Parses a string into an AST.
     */
    pub fn parse(&mut self) -> Result<Tree, ParseError> {
        self.program()
    }

    /**
     * This function is the main entry point of the parser.
     * 
     * Program
     *  : Literal
     *  ;
     */
    fn program(&mut self) -> Result<Tree, ParseError> {
        let node = self.literal()?;
        return Ok(Tree::Program {
            node_type: TreeNodeType::Program,
            body: Box::new(node),
        });
    }

    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal(&mut self) -> Result<Tree, ParseError> {
        let lookahead = self.lookahead.clone();
        match lookahead {
            Option::None => Err(ParseError {
                message: format(format_args!("Unexpected end of input, expected {:?} or {:?}!", TokenType::Number, TokenType::String))
            }),
            Option::Some(token) => {
                match token.token_type {
                    TokenType::Number => self.numeric_literal(),
                    TokenType::String => self.string_literal(),
                }
            }
        }
    }

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> Result<Tree, ParseError> {
        let token = self.eat(TokenType::Number)?;
        let parsed = token.value.parse().expect("Expected a numeric value!");
        return Ok(Tree::NumericLiteral { 
            node_type: TreeNodeType::NumericLiteral, 
            value: parsed 
        })
    }

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> Result<Tree, ParseError> {
        let token = self.eat(TokenType::String)?;

        // Removing quotes from start and end
        let value = String::from(&token.value[1..(token.value.len()-1)]);
        return Ok(Tree::StringLiteral { 
            node_type: TreeNodeType::StringLiteral, 
            value, 
        })
    }

    fn eat(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        let lookahead = self.lookahead.clone();
        match lookahead {
            Option::None => Err(ParseError {
                message: format(format_args!("Unexpected end of input, expected {:?}!", token_type)),
            }),
            Option::Some(token) => {
                if token.token_type != token_type {
                    return Err(ParseError {
                        message: format(format_args!("Unexpected token {:?}, expected {:?}!", token.token_type, token_type)),
                    });
                }
                
                // Advance to the next token.
                self.lookahead = self.tokenizer.get_next_token();

                Ok(token)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal_numeric() {
        let mut parser = Parser::new("42".to_owned());
        let node = parser.literal();
        assert!(node.is_ok());
        let expected = Tree::NumericLiteral { 
            node_type: TreeNodeType::NumericLiteral, 
            value: 42.0, 
        };
        assert_eq!(expected, node.unwrap());
    }

    #[test]
    fn test_parse_literal_string() {
        let mut parser = Parser::new("\"Hello\"".to_owned());
        let node = parser.literal();
        assert!(node.is_ok());
        let expected = Tree::StringLiteral { 
            node_type: TreeNodeType::StringLiteral, 
            value: "Hello".to_owned(), 
        };
        assert_eq!(expected, node.unwrap());
    }

    #[test]
    fn test_parse_literal_unexpected_none_token() {
        let mut parser = Parser::new("".to_owned());
        let expected = Err(ParseError {
            message: "Unexpected end of input, expected Number or String!".to_owned(),
        });
        assert_eq!(expected, parser.literal());
    }

    #[test]
    fn test_eat_token_type_number() {
        let mut parser = Parser::new("42".to_owned());
        let expected = Ok(Token {
            token_type: TokenType::Number,
            value: "42".to_owned(),
        });
        assert_eq!(expected, parser.eat(TokenType::Number));
    }

    #[test]
    fn test_eat_token_type_string() {
        let mut parser = Parser::new("\"Hello\"".to_owned());
        let expected = Ok(Token {
            token_type: TokenType::String,
            value: "\"Hello\"".to_owned(),
        });
        assert_eq!(expected, parser.eat(TokenType::String));
    }

    #[test]
    fn test_eat_unexpected_empty_input() {
        let mut parser = Parser::new("".to_owned());
        let expected = Err(ParseError {
            message: "Unexpected end of input, expected Number!".to_owned(),
        });
        assert_eq!(expected, parser.eat(TokenType::Number));
    }
}
