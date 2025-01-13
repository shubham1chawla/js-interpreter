use crate::prelude::*;

use super::eatable::Eatable;

pub trait LiteralParsable {
    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  | BooleanLiteral
     *  | NullLiteral
     *  | ThisLiteral
     *  | SuperLiteral
     *  ;
     */
    fn literal(&mut self) -> Result<Tree>;

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> Result<Tree>;

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> Result<Tree>;

    /**
     * BooleanLiteral
     *  : 'true'
     *  | 'false'
     *  ;
     */
    fn boolean_literal(&mut self) -> Result<Tree>;

    /**
     * NullLiteral
     *  : 'null'
     *  ;
     */
    fn null_literal(&mut self) -> Result<Tree>;

    /**
     * ThisLiteral
     *  : 'this'
     *  ;
     */
    fn this_literal(&mut self) -> Result<Tree>;

    /**
     * SuperLiteral
     *  : 'super'
     *  ;
     */
    fn super_literal(&mut self) -> Result<Tree>;
}

impl LiteralParsable for Parser {
    fn literal(&mut self) -> Result<Tree> {
        match self.lookahead.token_type {
            TokenType::Number => self.numeric_literal(),
            TokenType::String => self.string_literal(),
            TokenType::TrueKeyword | TokenType::FalseKeyword => self.boolean_literal(),
            TokenType::NullKeyword => self.null_literal(),
            TokenType::ThisKeyword => self.this_literal(),
            TokenType::SuperKeyword => self.super_literal(),
            _ => Err(Error::Syntax("Unexpected literal production!".to_string()))
        }
    }

    fn numeric_literal(&mut self) -> Result<Tree> {
        let token = self.eat(TokenType::Number)?;
        match token.value.parse() {
            Err(_) => Err(Error::Syntax("Expected a parsable numeric value!".to_string())),
            Ok(parsed) => Ok(Tree::NumericLiteral { value: parsed })
        }
    }

    fn string_literal(&mut self) -> Result<Tree> {
        let token = self.eat(TokenType::String)?;

        // Removing quotes from start and end
        let value = String::from(&token.value[1..(token.value.len()-1)]);
        return Ok(Tree::StringLiteral { value })
    }

    fn boolean_literal(&mut self) -> Result<Tree> {
        let token = match self.lookahead.token_type {
            TokenType::FalseKeyword => self.eat(TokenType::FalseKeyword)?,
            _ => self.eat(TokenType::TrueKeyword)?,
        };

        match token.value.parse::<bool>() {
            Err(_) => Err(Error::Syntax("Expected a parsable boolean value!".to_string())),
            Ok(parsed) => Ok(Tree::BooleanLiteral { value: parsed }),
        }
    }

    fn null_literal(&mut self) -> Result<Tree> {
        self.eat(TokenType::NullKeyword)?;
        Ok(Tree::NullLiteral)
    }

    fn this_literal(&mut self) -> Result<Tree> {
        self.eat(TokenType::ThisKeyword)?;
        Ok(Tree::ThisLiteral)
    }

    fn super_literal(&mut self) -> Result<Tree> {
        self.eat(TokenType::SuperKeyword)?;
        Ok(Tree::SuperLiteral)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

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
        let expected = Error::Syntax("Unexpected token EOF, expected SemiColon!".to_string());
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

    #[test]
    fn test_parse_this_literal() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::ThisLiteral),
                },
            ]),
        };
        assert_tree(expected, "this;");
    }

    #[test]
    fn test_parse_super_literal() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::SuperLiteral),
                },
            ]),
        };
        assert_tree(expected, "super;");
    }
}