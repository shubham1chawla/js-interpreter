use block::BlockStatementParsable;
use empty::EmptyStatementParsable;
use expression::ExpressionStatementParsable;
use ifconditional::IfStatementParsable;
use variable::VariableStatementParsable;

use super::*;

pub trait StatementListParsable {
    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_lookahead_type: TokenType) -> Result<Vec<Tree>, SyntaxError>;

    /**
     * Statement
     *  : EmptyStatement
     *  | BlockStatement
     *  | VariableStatement
     *  | IfStatement
     *  | ExpressionStatement
     *  ;
     */
    fn statement(&mut self) -> Result<Tree, SyntaxError>;
}

impl StatementListParsable for Parser {
    fn statement_list(&mut self, stop_lookahead_type: TokenType) -> Result<Vec<Tree>, SyntaxError> {
        let mut statement_list = vec![];

        while self.lookahead.token_type != stop_lookahead_type {
            statement_list.push(self.statement()?);
        }

        Ok(statement_list)
    }

    fn statement(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::SemiColon => self.empty_statement(),
            TokenType::CurlyBracketOpen => self.block_statement(),
            TokenType::LetKeyword => self.variable_statement(),
            TokenType::IfKeyword => self.if_statement(),
            _ => self.expression_statement(),
        }
    }
}

#[cfg(test)]
mod tests {
    use parsable::tests::assert_tree;

    use super::*;

    #[test]
    fn test_parse_statement_list() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]),
        };
        assert_tree(expected, "42;\"Hello\";");
    }
}