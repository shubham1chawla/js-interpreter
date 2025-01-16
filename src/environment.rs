use std::collections::HashMap;
use derive_builder::Builder;

use super::value::Value;

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
}
