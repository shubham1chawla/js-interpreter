use crate::prelude::*;

pub trait IdentifierEvalable {
    /**
     * Evaluate identifier AST Tree nodes.
     * + Identifier
     */
    fn eval_identifier(&self, identifier: &Tree, env_ref: &EnvRef) -> Result<Value>;
}

impl <'a> IdentifierEvalable for Interpreter<'a> {
    fn eval_identifier(&self, identifier: &Tree, env_ref: &EnvRef) -> Result<Value> {
        let depth = self.depth.get();
        if let Tree::Identifier { name } = identifier {
            // Extracting variable from environment
            let env = env_ref.borrow_mut();
            let value = env.lookup(name)?;
            println!("{}* {name} = {value:?}", " ".repeat(depth));
            return Ok(value);
        }
        Err(Error::Runtime(format!("Invalid identifier node: {identifier}")))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::prelude::*;
    use crate::interpreter::tests::*;

    #[test]
    fn test_variable_lookup() {
        let mut env = EnvironmentBuilder::default()
            .record(HashMap::from([
                ("abc".to_string(), Value::Number(25.0)),
            ]))
            .build()
            .unwrap();
        assert_value_env(Tree::Identifier { name: "abc".to_string() }, &mut env, Value::Number(25.0));
    }

    #[test]
    fn test_variable_not_defined() {
        let mut env = EnvironmentBuilder::default()
            .record(HashMap::from([
                ("abc".to_string(), Value::Number(25.0)),
            ]))
            .build()
            .unwrap();
        let error = Error::Runtime("Variable 'xyz' is not defined!".to_string());
        assert_runtime_error_env(Tree::Identifier { name: "xyz".to_string() }, &mut env, error);
    }
}
