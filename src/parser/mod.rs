use tree::{NodeKind, ValueNode};
pub mod tree;

pub struct Parser {
    content_string: String,
}

impl Parser {

    /**
     * Creates a new parser instance with code content as string.
     */
    pub fn new(string: String) -> Self {
        Self {
            content_string: string,
        }
    }

    /**
     * Parses a string into an AST.
     */
    pub fn parse(&self) -> ValueNode<f64> {
        return self.program();
    }

    /**
     * This function is the main entry point of the parser.
     * 
     * Program
     *  : NumericLiteral
     *  ;
     */
    fn program(&self) -> ValueNode<f64> {
        return self.numeric_literal();
    }

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&self) -> ValueNode<f64> {
        return ValueNode {
            kind: NodeKind::NumericLiteral,
            value: self.content_string.parse().expect("Expected a numeric value of 64 bits!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use super::*;

    #[test]
    fn test_parse_numeric_literal() {
        let expected = serde_json::to_string_pretty(ValueNode {
            kind: NodeKind::NumericLiteral,
            value: 42 as f64,
        }.borrow()).expect("Unable to convert ValueNode to json!");

        let parser = Parser::new("42".to_owned());
        let node = parser.parse();
        assert_eq!(expected, node.to_string());
    }
}
