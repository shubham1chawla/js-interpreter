use crate::prelude::*;

use super::function::FunctionExpressionParsable;
use super::identifier::IdentifierParsable;
use super::literal::LiteralParsable;
use super::new::NewExpressionParsable;
use super::paranthesized::ParanthesizedExpressionParsable;

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
    fn primary_expression(&mut self) -> Result<Tree>;
}

impl PrimaryExpressionParsable for Parser {
    fn primary_expression(&mut self) -> Result<Tree> {
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