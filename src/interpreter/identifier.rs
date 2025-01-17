use crate::prelude::*;

pub trait IdentifierEvalable {
    fn eval_identifier(&self, identifier: &Tree, env_ref: &EnvRef) -> Result<Value>;
}

impl IdentifierEvalable for Interpreter {
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
