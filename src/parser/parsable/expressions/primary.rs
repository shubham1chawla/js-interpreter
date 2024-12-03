use identifier::IdentifierParsable;
use literal::LiteralParsable;
use paranthesized::ParanthesizedExpressionParsable;

use super::*;

pub trait PrimaryExpressionParsable {
    /**
     * PrimaryExpression
     *  : ParanthesizedExpression
     *  | Literal
     *  | Identifier
     *  ;
     */
    fn primary_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl PrimaryExpressionParsable for Parser {
    fn primary_expression(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::CircleBracketOpen => self.paranthesized_expression(),
            TokenType::Number | TokenType::String | TokenType::TrueKeyword | TokenType::FalseKeyword | TokenType::NullKeyword => self.literal(),
            _ => self.identifier(),
        }
    }
}