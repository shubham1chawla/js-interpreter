use eatable::Eatable;
use expressions::assignment::AssignmentExpressionParsable;

use super::*;

pub trait ExpressionStatementParsable {
    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * Expression
     *  : AssignmentExpression
     *  ;
     */
    fn expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl ExpressionStatementParsable for Parser {
    fn expression_statement(&mut self) -> Result<Tree, SyntaxError> {
        let expression = self.expression()?;
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::ExpressionStatement { expression: Box::new(expression) })
    }

    fn expression(&mut self) -> Result<Tree, SyntaxError> {
        self.assignment_expression()
    }
}
