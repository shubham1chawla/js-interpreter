use tokenizer::*;
use tree::*;
pub use parsable::Parsable;

mod parsable;
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
}
