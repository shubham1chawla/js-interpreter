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

impl StatementEvalable for Interpreter {
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
