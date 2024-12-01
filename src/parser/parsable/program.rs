use statements::StatementListParsable;

use super::*;

pub trait ProgramParsable {
    /**
     * Program
     *  : StatementList
     *  ;
     */
    fn program(&mut self) -> Result<Tree, SyntaxError>;
}

impl ProgramParsable for Parser {
    fn program(&mut self) -> Result<Tree, SyntaxError> {
        let statement_list = self.statement_list(TokenType::EOF)?;
        Ok(Tree::Program { body: Box::new(statement_list) })
    }
}

#[cfg(test)]
mod tests {
    use parsable::tests::assert_tree;

    use super::*;

    #[test]
    fn test_parse_single_line_comments() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                }
            ]), 
        };
        assert_tree(expected, "// Comment \n 42;");
    }

    #[test]
    fn test_parse_multi_line_comments() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]), 
        };
        let content_string = "\
        /* Multi-line comment \n\
        * Hello
        */
        \"Hello\";";
        assert_tree(expected, content_string);
    }

    #[test]
    fn test_parse_empty_content() {
        let expected = Tree::Program { body: Box::new(vec![]) };
        assert_tree(expected, "");
    }
}