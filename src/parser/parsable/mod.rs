pub mod eatable;
pub mod parsable;

mod expressions;
mod identifier;
mod literal;
mod program;
mod statements;

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
