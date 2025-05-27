use colored::Colorize;

use crate::{analysis::Operator, value::ValueType};

pub const PRINT_SCAN_TOKENS: bool = false;
pub const DEBUG_TRACE_EXECUTION: bool = true;

pub const EXPECTED_SEMICOLON_MSG: &str = "Expected ';' at end of statement.";

pub fn print_error(line: u32, message: &str) {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    println!("{}{}{}{}", l, line, i, message);
}

#[derive(Debug)]
pub struct ParseErr {
    pub msg: String,
    pub line: u32,
}
impl ParseErr {
    pub fn new(line: u32, msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            line,
        }
    }
}

#[derive(Debug)]
pub struct EmitErr {
    pub msg: String,
    pub line: u32,
}
impl EmitErr {
    pub fn new(line: u32, msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            line,
        }
    }
}
pub struct SemanticError {
    ty: ErrType,
    line: u32,
}
impl SemanticError {
    pub fn new(line: u32, ty: ErrType) -> Self {
        Self { ty, line }
    }
}
pub enum ErrType {
    InvalidPrefix,
    InvalidInfix,
    UndefinedFunc(String),
    IncorrectArity(String, u8, u8),
    UndefinedVar(String),
    AlreadyDefinedVar(String),
    OpTypeMismatch(ValueType, Operator, ValueType),
    TypeMismatch(ValueType, ValueType),
}
impl SemanticError {
    pub fn print(&self) {
        let msg = match &self.ty {
            ErrType::InvalidPrefix => format!("invalid prefix."),
            ErrType::InvalidInfix => format!("invalid infix."),

            ErrType::IncorrectArity(name, expected, found) => {
                format!(
                    "Function '{}' expected {} arguments, but found {}.",
                    name, expected, found
                )
            }

            ErrType::UndefinedFunc(name) => format!("Function '{}' has not been defined.", name),
            ErrType::UndefinedVar(name) => {
                format!("Variable '{}' has not been defined in this scope.", name)
            }
            ErrType::AlreadyDefinedVar(name) => {
                format!(
                    "Variable '{}' has already been defined in this scope.",
                    name
                )
            }

            ErrType::OpTypeMismatch(expected, op, found) => {
                format!(
                    "Operator '{}' Expects type '{}', but found type '{}'.",
                    op, expected, found
                )
            }
            ErrType::TypeMismatch(expected, found) => {
                format!(
                    "Variable was given type '{}' but found type '{}'.",
                    expected, found
                )
            }
        };
        print_error(self.line, &msg);
    }
}
