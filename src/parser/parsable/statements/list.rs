use crate::prelude::*;

use super::block::BlockStatementParsable;
use super::class::ClassDeclarationParsable;
use super::empty::EmptyStatementParsable;
use super::expression::ExpressionStatementParsable;
use super::conditional::IfStatementParsable;
use super::function::FunctionDeclarationParsable;
use super::iteration::IterationStatementParsable;
use super::variable::VariableStatementParsable;

pub trait StatementListParsable {
    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_lookahead_type: TokenType) -> Result<Vec<Tree>>;

    /**
     * Statement
     *  : IterationStatement
     *  | FunctionDeclaration
     *  | ReturnKeyword
     *  | EmptyStatement
     *  | BlockStatement
     *  | VariableStatement
     *  | IfStatement
     *  | ClassDeclaration
     *  | ExpressionStatement
     *  ;
     * 
     * IterationStatement
     *  : WhileStatement
     *  | DoWhileStatement
     *  | ForStatement
     *  ;
     */
    fn statement(&mut self) -> Result<Tree>;
}

impl StatementListParsable for Parser {
    fn statement_list(&mut self, stop_lookahead_type: TokenType) -> Result<Vec<Tree>> {
        let mut statement_list = vec![];

        while self.lookahead.token_type != stop_lookahead_type {
            statement_list.push(self.statement()?);
        }

        Ok(statement_list)
    }

    fn statement(&mut self) -> Result<Tree> {
        match self.lookahead.token_type {
            TokenType::WhileKeyword | TokenType::DoKeyword | TokenType::ForKeyword => self.iteration_statement(),
            TokenType::FunctionKeyword => self.function_declaration(),
            TokenType::ReturnKeyword => self.return_statement(),
            TokenType::SemiColon => self.empty_statement(),
            TokenType::CurlyBracketOpen => self.block_statement(),
            TokenType::LetKeyword => self.variable_statement(),
            TokenType::IfKeyword => self.if_statement(),
            TokenType::ClassKeyword => self.class_delaration(),
            _ => self.expression_statement(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

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