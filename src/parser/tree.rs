use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NodeKind {
    NumericLiteral,
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::NumericLiteral => write!(f, "NumericLiteral"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ValueNode<T> 
where 
    T: fmt::Display,
    T: Serialize,
{
    pub kind: NodeKind,
    pub value: T,
}

impl<T> fmt::Display for ValueNode<T> 
where 
    T: fmt::Display,
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(&self).expect("Unable to serialize ValueNode!");
        return write!(f, "{}", json);
    }
}
