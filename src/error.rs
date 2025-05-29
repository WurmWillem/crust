use colored::Colorize;

use crate::{analysis_types::Operator, value::ValueType};

pub const PRINT_SCAN_TOKENS: bool = false;
pub const DEBUG_TRACE_EXECUTION: bool = false;

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
#[derive(Debug)]
pub struct SemanticErr {
    ty: ErrType,
    line: u32,
}
impl SemanticErr {
    pub fn new(line: u32, ty: ErrType) -> Self {
        Self { ty, line }
    }
}
#[derive(Debug)]
pub enum ErrType {
    InvalidPrefix,
    InvalidInfix,
    IndexNonArr(ValueType),
    UndefinedFunc(String),
    IncorrectArity(String, u8, u8),
    IncorrectReturnTy(ValueType, ValueType),
    UndefinedVar(String),
    AlreadyDefinedVar(String),
    OpTypeMismatch(ValueType, Operator, ValueType),
    TypeMismatch(ValueType, ValueType),
    ArrElTypeMismatch(ValueType, ValueType),
}
impl SemanticErr {
    pub fn print(&self) {
        let msg = match &self.ty {
            ErrType::InvalidPrefix => "invalid prefix.".to_string(),
            ErrType::InvalidInfix => "invalid infix.".to_string(),
            ErrType::IndexNonArr(ty) => format!("You can only index arrays, but you tried to index the type {}", ty),

            ErrType::IncorrectReturnTy(expected, found) => {
                format!(
                    "Function expected return type {}, but found type {}.",
                    expected, found
                )
            }
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
            ErrType::ArrElTypeMismatch(expected, found) => {
                format!(
                    "Not all elements in the array are of the same type. Array expected type '{}', but found type '{}'.",
                    expected, found
                )
            }
        };
        print_error(self.line, &msg);
    }
}
