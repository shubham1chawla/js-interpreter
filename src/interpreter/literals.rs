use crate::prelude::*;

pub trait LiteralEvalable {
    /**
     * Evaluates literal AST Tree nodes.
     * + NumericLiteral
     * + StringLiteral
     * + NullLiteral
     * + BooleanLiteral
     */
    fn eval_literal(&self, literal: &Tree) -> Result<Value>;
}

impl <'a> LiteralEvalable for Interpreter<'a> {
    fn eval_literal(&self, literal: &Tree) -> Result<Value> {
        match literal {
            Tree::NumericLiteral { value } => Ok(Value::Number(*value)),
            Tree::StringLiteral { value } => Ok(Value::String((*value).clone())),
            Tree::NullLiteral => Ok(Value::Null),
            Tree::BooleanLiteral { value } => Ok(Value::Boolean(*value)),
            _ => Err(Error::Runtime(
                format!("Unimplemented literal node: {literal}")
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::interpreter::tests::*;

    #[test]
    fn test_numeric_literal() {
        assert_value(Tree::NumericLiteral { value: 10.0 }, Value::Number(10.0));
    }

    #[test]
    fn test_string_literal() {
        assert_value(Tree::StringLiteral { value: "Hello!".to_string() }, Value::String("Hello!".to_string()));
    }

    #[test]
    fn test_null_literal() {
        assert_value(Tree::NullLiteral, Value::Null);
    }

    #[test]
    fn test_boolean_literal() {
        assert_value(Tree::BooleanLiteral { value: true }, Value::Boolean(true));
    }
}
