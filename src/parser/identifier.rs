use crate::prelude::*;

pub trait IdentifierParsable {
    /**
     * Identifier
     *  : IDENTIFIER
     *  ;
     */
    fn identifier(&mut self) -> Result<Tree>;
}

impl IdentifierParsable for Parser {
    fn identifier(&mut self) -> Result<Tree> {
        let name = self.eat(TokenType::Identifier)?.value;
        Ok(Tree::Identifier { name })
    }
}