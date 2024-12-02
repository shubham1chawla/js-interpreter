use super::*;

pub trait EmptyStatementParsable {
    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    fn empty_statement(&mut self) -> Result<Tree, SyntaxError>;
}

impl EmptyStatementParsable for Parser {
    fn empty_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::EmptyStatement)
    }
}

#[cfg(test)]
mod tests {
    use parsable::tests::assert_tree;

    use super::*;

    #[test]
    fn test_parse_simple_empty_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::EmptyStatement,
            ]),
        };
        assert_tree(expected, ";");
    }

    #[test]
    fn test_parse_empty_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::EmptyStatement,
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::EmptyStatement,
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]),
        };
        assert_tree(expected, ";\n42;\n;\n'Hello';");
    }
}