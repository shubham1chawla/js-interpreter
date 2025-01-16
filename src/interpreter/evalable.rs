use crate::prelude::*;

use crate::interpreter::expressions::ExpressionEvalable;
use crate::interpreter::literals::LiteralEvalable;
use crate::interpreter::statements::StatementEvalable;

pub trait Evalable {
    /**
     * Evaluates AST Tree set while constructing Interpreter instance.
     */
    fn eval(&self) -> Result<Value>;

    /**
     * Evaluates arbitrary AST Tree node using provided Enviornment instance.
     */
    fn eval_tree(&self, tree: &Tree, env_ref: &EnvironmentRefCell) -> Result<Value>;
}

impl Evalable for Interpreter {
    fn eval(&self) -> Result<Value> {
        self.eval_tree(&self.tree, &self.env)
    }

    fn eval_tree(&self, tree: &Tree, env_ref: &EnvironmentRefCell) -> Result<Value> {
        let depth = self.increment_depth();

        println!("{}> {tree}", " ".repeat(depth - 1));
        let value = match tree {
            // ----- PROGRAM -----
            Tree::Program { body } => {
                for statement in body {
                    self.eval_tree(statement, env_ref)?;
                }
                Ok(Value::Undefined)
            },

            // ----- EXPRESSIONS -----
            Tree::BinaryExpression { .. } => self.eval_expression(tree, env_ref),

            // ----- STATEMENTS -----
            Tree::ExpressionStatement { .. } 
            | Tree::VariableStatement { .. } 
            | Tree::VariableDeclaration { .. } => self.eval_statement(tree, env_ref),

            // ----- LITERALS -----
            Tree::NumericLiteral { .. } 
            | Tree::StringLiteral { .. } => self.eval_literals(tree),

            // ----- UNIMPLEMENTED -----
            _ => Err(Error::Runtime(format!("Unimplemented tree node: {tree}")))
        }?;

        self.decrement_depth();
        Ok(value)
    }
}
