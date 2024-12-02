use eatable::Eatable;

use super::*;

pub trait IdentifierParsable {
    /**
     * Identifier
     *  : IDENTIFIER
     *  ;
     */
    fn identifier(&mut self) -> Result<Tree, SyntaxError>;
}

impl IdentifierParsable for Parser {
    fn identifier(&mut self) -> Result<Tree, SyntaxError> {
        let name = self.eat(TokenType::Identifier)?.value;
        Ok(Tree::Identifier { name })
    }
}