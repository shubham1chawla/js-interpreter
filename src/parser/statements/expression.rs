use crate::prelude::*;

use super::expressions::assignment::AssignmentExpressionParsable;

pub trait ExpressionStatementParsable {
    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self) -> Result<Tree>;

    /**
     * Expression
     *  : AssignmentExpression
     *  ;
     */
    fn expression(&mut self) -> Result<Tree>;
}

impl ExpressionStatementParsable for Parser {
    fn expression_statement(&mut self) -> Result<Tree> {
        let expression = self.expression()?;
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::ExpressionStatement { expression: Box::new(expression) })
    }

    fn expression(&mut self) -> Result<Tree> {
        self.assignment_expression()
    }
}
