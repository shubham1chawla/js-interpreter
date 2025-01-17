use crate::prelude::*;

pub trait ExpressionEvalable {
    /**
     * Evaluate expression AST Tree nodes.
     * + BinaryExpression
     */
    fn eval_expression(&self, expr: &Tree, env_ref: &EnvRef) -> Result<Value>;
}

impl <'a> ExpressionEvalable for Interpreter<'a> {
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
                    _ => Err(Error::Runtime(format!("Unimplemented operator: {operator}"))),
                }
            },
            _ => Err(Error::Runtime(format!("Unimplemented expression node: {expr}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::interpreter::tests::*;

    #[test]
    fn test_binary_expression_1() {
        let tree = Tree::BinaryExpression {
            operator: "+".to_string(),
            left: Box::new(Tree::NumericLiteral { value: 10.0 }),
            right: Box::new(Tree::BinaryExpression {
                operator: "*".to_string(),
                left: Box::new(Tree::NumericLiteral { value: 10.0 }),
                right: Box::new(Tree::NumericLiteral { value: 20.0 }),
            }),
        };
        assert_value(tree, Value::Number(210.0));
    }

    #[test]
    fn test_binary_expression_2() {
        let tree = Tree::BinaryExpression {
            operator: "-".to_string(),
            left: Box::new(Tree::BinaryExpression {
                operator: "+".to_string(),
                left: Box::new(Tree::NumericLiteral { value: 20.0 }),
                right: Box::new(Tree::NumericLiteral { value: 30.0 }),
            }),
            right: Box::new(Tree::BinaryExpression {
                operator: "/".to_string(),
                left: Box::new(Tree::NumericLiteral { value: 25.0 }),
                right: Box::new(Tree::NumericLiteral { value: 5.0 }),
            }),
        };
        assert_value(tree, Value::Number(45.0));
    }

    #[test]
    fn test_unimplemented_operator_binary_expression() {
        let tree = Tree::BinaryExpression {
            operator: "^".to_string(),
            left: Box::new(Tree::NumericLiteral { value: 10.0 }),
            right: Box::new(Tree::NumericLiteral { value: 10.0 }),
        };
        let error = Error::Runtime("Unimplemented operator: ^".to_string());
        assert_runtime_error(tree, error);
    }
}
