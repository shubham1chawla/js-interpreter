use super::*;

pub trait Parsable {
    /**
     * Parses a string into an AST.
     */
    fn parse(&mut self) -> Result<Tree, SyntaxError>;
}

impl Parsable for Parser {
    fn parse(&mut self) -> Result<Tree, SyntaxError> {
        self.program()
    }
}
