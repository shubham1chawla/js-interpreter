use member::MemberExpressionParsable;

use super::*;

pub trait LeftHandSideExpressionParsable {
    /**
     * LeftHandSideExpression
     *  : MemberExpression
     *  ;
     */
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl LeftHandSideExpressionParsable for Parser {
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError> {
        self.member_expression()
    }
}
