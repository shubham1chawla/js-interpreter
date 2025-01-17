use crate::prelude::*;

pub trait StatementEvalable {
    /**
     * Evaluates statement AST Tree nodes.
     * + ExpressionStatement
     * + VariableStatement
     * + VariableDeclaration
     */
    fn eval_statement(&self, statement: &Tree, env_ref: &EnvRef) -> Result<Value>;
}

impl <'a> StatementEvalable for Interpreter<'a> {
    fn eval_statement(&self, statement: &Tree, env_ref: &EnvRef) -> Result<Value> {
        let depth = self.depth.get();
        match statement {
            Tree::ExpressionStatement { expression } => self.eval_tree(expression, env_ref),
            Tree::VariableStatement { declarations } => {
                for declaration in declarations {
                    self.eval_tree(declaration, env_ref)?;
                }
                Ok(Value::Undefined)
            },
            Tree::VariableDeclaration { identifier, init } => {
                // Checking if the node conforms to Identifier type
                if let Tree::Identifier { name } = identifier.as_ref() {

                    // Extracting variable value
                    let value = match init.as_ref() {
                        None => Value::Undefined,
                        Some(init) => self.eval_tree(init, env_ref)?,
                    };

                    // Defining variable in the environment
                    println!("{}+ {name} = {value:?}", " ".repeat(depth));
                    let mut env = env_ref.borrow_mut();
                    env.define(name.clone(), value);
                } else {
                    return Err(Error::Runtime(format!("Unable to extract identifier from node: {identifier:?}")));
                };
                Ok(Value::Undefined)
            },
            _ => Err(Error::Runtime(format!("Unimplemented statement node: {statement}")))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::prelude::*;
    use crate::interpreter::tests::*;

    #[test]
    fn test_variable_declaration() {
        let mut env = EnvironmentBuilder::default().build().unwrap();
        let tree = Tree::ExpressionStatement {
            expression: Box::new(Tree::VariableStatement {
                declarations: vec![
                    Tree::VariableDeclaration {
                        identifier: Box::new(Tree::Identifier { name: "abc".to_string() }),
                        init: Box::new(Some(Tree::NumericLiteral { value: 10.0 })),
                    },
                    Tree::VariableDeclaration {
                        identifier: Box::new(Tree::Identifier { name: "xyz".to_string() }),
                        init: Box::new(None),
                    },
                ],
            }),
        };
        assert_value_env(tree, &mut env, Value::Undefined);

        let result = env.lookup(&"abc".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Number(10.0));

        let result = env.lookup(&"xyz".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Undefined);
    }

    #[test]
    fn test_expression_statement_1() {
        let mut env = EnvironmentBuilder::default()
            .record(HashMap::from([
                ("abc".to_string(), Value::Number(20.0)),
            ]))
            .build()
            .unwrap();

        // abc + '!';
        let tree = Tree::ExpressionStatement {
            expression: Box::new(Tree::BinaryExpression {
                operator: "+".to_string(),
                left: Box::new(Tree::Identifier { name: "abc".to_string() }),
                right: Box::new(Tree::StringLiteral { value: "!".to_string() }),
            }),
        };

        assert_value_env(tree, &mut env, Value::String("20!".to_string()));
    }
}
