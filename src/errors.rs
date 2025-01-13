#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("SyntaxError: {}", .0)]
    Syntax(String),
}
