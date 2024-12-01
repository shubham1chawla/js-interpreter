use std::fmt::format;

use program::ProgramParsable;

use super::*;

mod expressions;
mod identifier;
mod literal;
mod program;
mod statements;

pub trait Parsable {
    /**
     * Parses a string into an AST.
     */
    fn parse(&mut self) -> Result<Tree, SyntaxError>;
}

pub trait Eatable {
    /**
     * Eats a token and advances the lookahead token.
     * Throws a Syntax error if lookahead doesn't match supplied token.
     */
    fn eat(&mut self, token_type: TokenType) -> Result<Token, SyntaxError>;
}

impl Parsable for Parser {
    fn parse(&mut self) -> Result<Tree, SyntaxError> {
        self.program()
    }
}

impl Eatable for Parser {
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
    use parsable::Parsable;
    use tree::Tree;

    use super::*;

    pub fn assert_tree(expected: Tree, content_string: &str) {
        let parser_result = Parser::new(content_string.to_owned());
        assert!(parser_result.is_ok());
        let mut parser = parser_result.unwrap();
        let tree_result = parser.parse();
        assert!(tree_result.is_ok());
        assert_eq!(expected, tree_result.unwrap());
    }

    pub fn assert_syntax_error(expected: SyntaxError, content_string: &str) {
        let parser_result = Parser::new(content_string.to_owned());
        assert!(parser_result.is_ok());
        let mut parser = parser_result.unwrap();
        let tree_result = parser.parse();
        assert!(tree_result.is_err());
        assert_eq!(expected, tree_result.unwrap_err());
    }
}
