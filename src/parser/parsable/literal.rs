use eatable::Eatable;

use super::*;

pub trait LiteralParsable {
    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  | BooleanLiteral
     *  | NullLiteral
     *  ;
     */
    fn literal(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * BooleanLiteral
     *  : 'true'
     *  | 'false'
     *  ;
     */
    fn boolean_literal(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * NullLiteral
     *  : 'null'
     *  ;
     */
    fn null_literal(&mut self) -> Result<Tree, SyntaxError>;
}

impl LiteralParsable for Parser {
    fn literal(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::Number => self.numeric_literal(),
            TokenType::String => self.string_literal(),
            TokenType::TrueKeyword | TokenType::FalseKeyword => self.boolean_literal(),
            TokenType::NullKeyword => self.null_literal(),
            _ => Err(SyntaxError {
                message: String::from("Unexpected literal production!"),
            })
        }
    }

    fn numeric_literal(&mut self) -> Result<Tree, SyntaxError> {
        let token = self.eat(TokenType::Number)?;
        match token.value.parse() {
            Err(_) => Err(SyntaxError {
                message: String::from("Expected a parsable numeric value!"),
            }),
            Ok(parsed) => Ok(Tree::NumericLiteral { value: parsed })
        }
    }

    fn string_literal(&mut self) -> Result<Tree, SyntaxError> {
        let token = self.eat(TokenType::String)?;

        // Removing quotes from start and end
        let value = String::from(&token.value[1..(token.value.len()-1)]);
        return Ok(Tree::StringLiteral { value })
    }

    fn boolean_literal(&mut self) -> Result<Tree, SyntaxError> {
        let token = match self.lookahead.token_type {
            TokenType::FalseKeyword => self.eat(TokenType::FalseKeyword)?,
            _ => self.eat(TokenType::TrueKeyword)?,
        };

        match token.value.parse::<bool>() {
            Err(_) => Err(SyntaxError {
                message: String::from("Expected a parsable boolean value!"),
            }),
            Ok(parsed) => Ok(Tree::BooleanLiteral { value: parsed }),
        }
    }

    fn null_literal(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::NullKeyword)?;
        Ok(Tree::NullLiteral)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parsable::tests::{assert_syntax_error, assert_tree};

    use super::*;

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

    #[test]
    fn test_parse_missing_semicolon() {
        let expected = SyntaxError {
            message: String::from("Unexpected token EOF, expected SemiColon!"),
        };
        assert_syntax_error(expected, "42");
    }

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

    #[test]
    fn test_parse_true_literal() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::BooleanLiteral { value: true }),
                },
            ]),
        };
        assert_tree(expected, "true;");
    }

    #[test]
    fn test_parse_false_literal() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::BooleanLiteral { value: false }),
                },
            ]),
        };
        assert_tree(expected, "false;");
    }

    #[test]
    fn test_parse_null_literal() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::NullLiteral),
                },
            ]),
        };
        assert_tree(expected, "null;");
    }
}