use crate::prelude::*;

pub trait LiteralEvalable {
    /**
     * Evaluates literal AST Tree nodes.
     * + NumericLiteral
     * + StringLiteral
     * + NullLiteral
     */
    fn eval_literal(&self, literal: &Tree) -> Result<Value>;
}

impl LiteralEvalable for Interpreter {
    fn eval_literal(&self, literal: &Tree) -> Result<Value> {
        match literal {
            Tree::NumericLiteral { value } => Ok(Value::Number(*value)),
            Tree::StringLiteral { value } => Ok(Value::String((*value).clone())),
            Tree::NullLiteral => Ok(Value::Null),
            _ => Err(Error::Runtime(
                format!("Unimplemented literal node: {literal}")
            ))
        }
    }
}
