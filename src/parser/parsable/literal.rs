use super::*;

pub trait LiteralParsable {
    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
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
}

impl LiteralParsable for Parser {
    fn literal(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::Number => self.numeric_literal(),
            _ => self.string_literal(),
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
}

#[cfg(test)]
mod tests {
    use parsable::tests::{assert_syntax_error, assert_tree};

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
}