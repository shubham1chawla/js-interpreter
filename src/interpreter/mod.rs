pub use evalable::Evalable;

mod evalable;
mod value;

/**
 * AST Tree-walking interpreter implementation.
 */
pub struct Interpreter;

impl Interpreter {
    /**
     * Creates a new interpreter with AST Tree.
     */
    pub fn new() -> Self {
        Self {}
    }
}
