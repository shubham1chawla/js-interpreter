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
pub struct Interpreter<'a> {
    tree: &'a Tree,
    env_ref: RefCell<&'a mut Environment>,
    depth: Cell<usize>,
}

impl <'a> Interpreter<'a> {
    /**
     * Creates a new interpreter with AST Tree.
     */
    pub fn new(tree: &'a Tree, env: &'a mut Environment) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    pub fn assert_value(tree: Tree, value: Value) {
        let mut env = EnvironmentBuilder::default().build().unwrap();
        assert_value_env(tree, &mut env, value);
    }

    pub fn assert_value_env(tree: Tree, env: &mut Environment, value: Value) {
        let interpreter = Interpreter::new(&tree, env);
        let result = interpreter.eval();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), value);
    }

    pub fn assert_runtime_error(tree: Tree, error: Error) {
        let mut env = EnvironmentBuilder::default().build().unwrap();
        assert_runtime_error_env(tree, &mut env, error);
    }

    pub fn assert_runtime_error_env(tree: Tree, env: &mut Environment, error: Error) {
        let interpreter = Interpreter::new(&tree, env);
        let result = interpreter.eval();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }
}
