use crate::prelude::*;

use std::collections::HashMap;
use derive_builder::Builder;

#[derive(Builder, Clone, Debug)]
pub struct Environment {
    #[builder(default="HashMap::new()")]
    record: HashMap<String, Value>,

    #[builder(default="Option::None")]
    parent: Option<Box<Environment>>,
}

impl Environment {
    /**
     * Creates a variable with the given name and value.
     */
    pub fn define(&mut self, name: String, value: Value) {
        self.record.insert(name, value);
    }

    /**
     * Returns the value of a defined variable, or returns
     * an error if the variable is not defined.
     */
    pub fn lookup(&self, name: &String) -> Result<Value> {
        match self.record.get(name) {
            None => Err(Error::Runtime(format!("Variable '{name}' is not defined!"))),
            Some(value) => Ok(value.clone()),
        }
    }
}
