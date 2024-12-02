use identifier::IdentifierParsable;

use super::*;

pub trait LeftHandSideExpressionParsable {
    /**
     * LeftHandSideExpression
     *  : Identifier
     *  ;
     */
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl LeftHandSideExpressionParsable for Parser {
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError> {
        self.identifier()
    }
}
