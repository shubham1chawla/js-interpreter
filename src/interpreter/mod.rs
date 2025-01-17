use crate::prelude::*;

use std::cell::{Cell, RefCell};

pub use evalable::Evalable;

mod expressions;
mod identifier;
mod evalable;
mod literals;
mod statements;

/**
 * AST Tree-walking interpreter implementation.
 */
pub struct Interpreter {
    tree: Tree,
    env_ref: RefCell<Environment>,
    depth: Cell<usize>,
}

impl Interpreter {
    /**
     * Creates a new interpreter with AST Tree.
     */
    pub fn new(tree: Tree) -> Self {
        // Creating global environment
        let env = EnvironmentBuilder::default()
            .build()
            .unwrap();
        Self {
            tree,
            env_ref: RefCell::new(env),
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
