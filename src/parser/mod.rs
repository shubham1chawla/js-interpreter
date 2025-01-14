use crate::prelude::*;

pub use eatable::Eatable;
pub use parsable::Parsable;

mod expressions;
mod statements;
mod eatable;
mod identifier;
mod literal;
mod parsable;
mod program;

/**
 * Recursive decent parser implementation
 */
pub struct Parser {
    tokenizer: Tokenizer,
    lookahead: Token,
}

impl Parser {
    /**
     * Creates a new parser instance with code content as string.
     */
    pub fn new(content_string: String) -> Result<Self> {
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
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    pub fn assert_tree(expected: Tree, content_string: &str) {
        let parser_result = Parser::new(content_string.to_owned());
        assert!(parser_result.is_ok());
        let mut parser = parser_result.unwrap();
        let tree_result = parser.parse();
        assert!(tree_result.is_ok());
        assert_eq!(expected, tree_result.unwrap());
    }

    pub fn assert_syntax_error(expected: Error, content_string: &str) {
        let parser_result = Parser::new(content_string.to_owned());
        assert!(parser_result.is_ok());
        let mut parser = parser_result.unwrap();
        let tree_result = parser.parse();
        assert!(tree_result.is_err());
        assert_eq!(expected, tree_result.unwrap_err());
    }
}
