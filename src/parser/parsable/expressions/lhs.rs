use primary::PrimaryExpressionParsable;

use super::*;

pub trait LeftHandSideExpressionParsable {
    /**
     * LeftHandSideExpression
     *  : PrimaryExpression
     *  ;
     */
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl LeftHandSideExpressionParsable for Parser {
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError> {
        self.primary_expression()
    }
}
