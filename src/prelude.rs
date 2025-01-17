pub use crate::interpreter::{Evalable, Interpreter};
pub use crate::parser::{Eatable, Parsable, Parser};
pub use crate::environment::{Environment, EnvironmentBuilder};
pub use crate::errors::Error;
pub use crate::tokenizer::{Token, TokenType, Tokenizer};
pub use crate::tree::Tree;
pub use crate::value::Value;

pub type Result<T> = core::result::Result<T, Error>;
pub type EnvRef<'a> = std::cell::RefCell<&'a mut Environment>;
