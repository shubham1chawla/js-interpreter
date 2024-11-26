use tree::Tree;

mod tree;

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
    pub fn parse(&self) -> Tree {
        return self.program();
    }

    /**
     * This function is the main entry point of the parser.
     * 
     * Program
     *  : NumericLiteral
     *  ;
     */
    fn program(&self) -> Tree {
        return self.numeric_literal();
    }

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&self) -> Tree {
        let parsed = self.content_string.parse().expect("Expected a numerical value!");
        return Tree::NumericLiteral(Box::new(parsed));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numeric_literal() {
        let parser = Parser::new("42".to_owned());
        let node = parser.parse();
        assert_eq!(Tree::NumericLiteral(Box::new(42.0)), node);
    }
}
