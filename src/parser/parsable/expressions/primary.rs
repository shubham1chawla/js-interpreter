use lhs::LeftHandSideExpressionParsable;
use literal::LiteralParsable;
use paranthesized::ParanthesizedExpressionParsable;

use super::*;

pub trait PrimaryExpressionParsable {
    /**
     * PrimaryExpression
     *  : Literal
     *  | ParanthesizedExpression
     *  | LeftHandSideExpression
     *  ;
     */
    fn primary_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl PrimaryExpressionParsable for Parser {
    fn primary_expression(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::Number | TokenType::String | TokenType::TrueKeyword | TokenType::FalseKeyword | TokenType::NullKeyword => self.literal(),
            TokenType::CircleBracketOpen => self.paranthesized_expression(),
            _ => self.left_hand_side_expression(),
        }
    }
}