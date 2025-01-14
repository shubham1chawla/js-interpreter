use crate::prelude::*;

use super::program::ProgramParsable;

pub trait Parsable {
    /**
     * Parses a string into an AST.
     */
    fn parse(&mut self) -> Result<Tree>;
}

impl Parsable for Parser {
    fn parse(&mut self) -> Result<Tree> {
        self.program()
    }
}
