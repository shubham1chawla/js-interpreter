use call::CallExpressionParsable;

use super::*;

pub trait LeftHandSideExpressionParsable {
    /**
     * LeftHandSideExpression
     *  : CallMemberExpression
     *  ;
     */
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl LeftHandSideExpressionParsable for Parser {
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError> {
        self.call_member_expression()
    }
}
