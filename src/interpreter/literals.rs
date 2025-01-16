use crate::prelude::*;

pub trait LiteralEvalable {
    /**
     * Evaluates literal AST Tree nodes.
     * + NumericLiteral
     * + StringLiteral
     */
    fn eval_literals(&self, literal: &Tree) -> Result<Value>;
}

impl LiteralEvalable for Interpreter {
    fn eval_literals(&self, literal: &Tree) -> Result<Value> {
        match literal {
            Tree::NumericLiteral { value } => Ok(Value::Number(*value)),
            Tree::StringLiteral { value } => Ok(Value::String((*value).clone())),
            _ => Err(Error::Runtime(
                format!("Unimplemented literal node: {literal}")
            ))
        }
    }
}
