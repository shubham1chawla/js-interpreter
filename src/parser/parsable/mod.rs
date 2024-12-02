use program::ProgramParsable;

use super::*;

mod expressions;
mod eatable;
mod identifier;
mod literal;
pub mod parsable;
mod program;
mod statements;

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
