use crate::prelude::*;

use super::call::CallExpressionParsable;

pub trait LeftHandSideExpressionParsable {
    /**
     * LeftHandSideExpression
     *  : CallMemberExpression
     *  ;
     */
    fn left_hand_side_expression(&mut self) -> Result<Tree>;
}

impl LeftHandSideExpressionParsable for Parser {
    fn left_hand_side_expression(&mut self) -> Result<Tree> {
        self.call_member_expression()
    }
}
