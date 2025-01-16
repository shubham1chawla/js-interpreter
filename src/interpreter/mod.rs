use crate::prelude::*;

use std::cell::{Cell, RefCell};

pub use evalable::Evalable;

mod expressions;
mod evalable;
mod literals;
mod statements;

/**
 * AST Tree-walking interpreter implementation.
 */
pub struct Interpreter {
    tree: Tree,
    env: EnvironmentRefCell,
    depth: Cell<usize>,
}

impl Interpreter {
    /**
     * Creates a new interpreter with AST Tree.
     */
    pub fn new(tree: Tree) -> Self {
        let env = EnvironmentBuilder::default().build().unwrap();
        Self {
            tree,
            env: RefCell::new(env),
            depth: Cell::new(0),
        }
    }

    fn increment_depth(&self) -> usize {
        let depth = self.depth.get() + 1;
        self.depth.set(depth);
        depth
    }

    fn decrement_depth(&self) -> usize {
        let depth = self.depth.get() - 1;
        self.depth.set(depth);
        depth
    }
}
