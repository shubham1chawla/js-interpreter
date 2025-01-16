use crate::prelude::*;

use std::fmt::format;

use super::value::Value;

pub trait Evalable {
    /**
     * Recursively evaluates AST Tree nodes.
     */
    fn eval(&self, node: Tree) -> Result<Value>;
}

impl Evalable for Interpreter {
    fn eval(&self, node: Tree) -> Result<Value> {
        println!("> {node}");
        match node {
            // ----- PROGRAM -----
            Tree::Program { body } => {
                for statement in body {
                    let result = self.eval(statement)?;
                    println!("< {result:?}");
                }
                Ok(Value::Undefined)
            },

            // ----- EXPRESSIONS -----
            Tree::BinaryExpression { operator, left, right } => {
                // Reducing left and right operands
                let lvalue = self.eval(*left)?;
                let rvalue = self.eval(*right)?;
                match operator.as_str() {
                    "+" => Ok(lvalue + rvalue),
                    "-" => Ok(lvalue - rvalue),
                    "*" => Ok(lvalue * rvalue),
                    "/" => Ok(lvalue / rvalue),
                    _ => Err(Error::Runtime(
                        format(format_args!("Unknown operator: {operator}"))
                    )),
                }
            },

            // ----- STATEMENTS -----
            Tree::ExpressionStatement { expression } => self.eval(*expression),

            // ----- LITERALS -----
            Tree::NumericLiteral { value } => Ok(Value::Number(value)),
            Tree::StringLiteral { value } => Ok(Value::String(value)),

            // ----- UNIMPLEMENTED -----
            _ => Err(Error::Runtime(
                format(format_args!("Unimplemented node: {}", node))
            ))
        }
    }
}
