use crate::prelude::*;

pub trait ExpressionEvalable {
    /**
     * Evaluate expression AST Tree nodes.
     * + BinaryExpression
     */
    fn eval_expression(&self, expr: &Tree, env_ref: &EnvRef) -> Result<Value>;
}

impl ExpressionEvalable for Interpreter {
    fn eval_expression(&self, expr: &Tree, env_ref: &EnvRef) -> Result<Value> {
        match expr {
            Tree::BinaryExpression { operator, left, right } => {
                // Reducing left and right operands
                let lvalue = self.eval_tree(left, env_ref)?;
                let rvalue = self.eval_tree(right, env_ref)?;
                match operator.as_str() {
                    "+" => Ok(lvalue + rvalue),
                    "-" => Ok(lvalue - rvalue),
                    "*" => Ok(lvalue * rvalue),
                    "/" => Ok(lvalue / rvalue),
                    _ => Err(Error::Runtime(format!("Unknown operator: {operator}"))),
                }
            },
            _ => Err(Error::Runtime(format!("Unimplemented expression node: {expr}"))),
        }
    }
}
