use crate::prelude::*;

use super::statements::list::StatementListParsable;

pub trait ProgramParsable {
    /**
     * Program
     *  : StatementList
     *  ;
     */
    fn program(&mut self) -> Result<Tree>;
}

impl ProgramParsable for Parser {
    fn program(&mut self) -> Result<Tree> {
        let body = self.statement_list(TokenType::Eof)?;
        Ok(Tree::Program { body })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::tests::*;

    #[test]
    fn test_parse_single_line_comments() {
        let expected = Tree::Program { 
            body: vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                }
            ], 
        };
        assert_tree(expected, "// Comment \n 42;");
    }

    #[test]
    fn test_parse_multi_line_comments() {
        let expected = Tree::Program { 
            body: vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ], 
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
        let expected = Tree::Program { body: vec![] };
        assert_tree(expected, "");
    }
}