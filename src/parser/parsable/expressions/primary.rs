use function::FunctionExpressionParsable;
use identifier::IdentifierParsable;
use literal::LiteralParsable;
use new::NewExpressionParsable;
use paranthesized::ParanthesizedExpressionParsable;

use super::*;

pub trait PrimaryExpressionParsable {
    /**
     * PrimaryExpression
     *  : ParanthesizedExpression
     *  | FunctionExpression
     *  | NewExpression
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
            TokenType::FunctionKeyword => self.function_expression(),
            TokenType::NewKeyword => self.new_expression(),
            TokenType::Number 
            | TokenType::String 
            | TokenType::TrueKeyword 
            | TokenType::FalseKeyword 
            | TokenType::NullKeyword 
            | TokenType::ThisKeyword 
            | TokenType::SuperKeyword => self.literal(),
            _ => self.identifier(),
        }
    }
}