use colored::Colorize;

use crate::{analysis_types::Operator, value::ValueType};

pub const PRINT_TOKENS: bool = false;
pub const DEBUG_TRACE_EXECUTION: bool = false;
pub const PRINT_HEAP: bool = false;

pub fn print_error(line: u32, msg: &str) {
    let l = "[line ".blue();
    // let line = line.to_string().magenta();
    let closing_bracket = "]".blue();
    let i = " Error: ".bright_red();
    let msg = msg.yellow();
    println!("{}{}{}{}{}", l, line, closing_bracket, i, msg);
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
    ty: SemErrType,
    line: u32,
}
impl SemanticErr {
    pub fn new(line: u32, ty: SemErrType) -> Self {
        Self { ty, line }
    }
}
#[derive(Debug)]
pub enum SemErrType {
    InvalidPrefix,
    InvalidInfix,
    InvalidPropertyAccess(ValueType),
    InvalidProperty(String, String),
    IndexNonArr(ValueType),
    AssignArrTypeMismatch(ValueType, ValueType),
    UndefinedFunc(String),
    UndefinedStruct(String),
    IncorrectArity(String, u8, u8),
    IncorrectReturnTy(ValueType, ValueType),
    UndefinedVar(String),
    AlreadyDefinedVar(String),
    AlreadyDefinedFunc(String),
    AlreadyDefinedStruct(String),
    OpTypeMismatch(ValueType, Operator, ValueType),
    VarDeclTypeMismatch(ValueType, ValueType),
    FieldTypeMismatch(ValueType, ValueType),
    ArrElTypeMismatch(ValueType, ValueType),
}
impl SemanticErr {
    pub fn print(&self) {
        //dbg!(&self.ty);
        let msg = match &self.ty {
            SemErrType::InvalidPrefix => "invalid prefix.".to_string(),
            SemErrType::InvalidInfix => "invalid infix.".to_string(),
            SemErrType::InvalidPropertyAccess(ty) => {
                format!(
                    "You can only access properties of instances, but you tried to access properties of type '{}'.",
                    ty
                )
            }
            SemErrType::InvalidProperty(name, property) => {
                format!(
                    "'{}' is not a valid property of Struct '{}'.",
                    property, name
                )
            }
            SemErrType::IndexNonArr(ty) => format!(
                "You can only index arrays, but you tried to index the type '{}'.",
                ty
            ),

            SemErrType::AssignArrTypeMismatch(expected, found) => {
                format!(
                    "Array is of type '[{}]', but you tried to assign a value of type '{}' to one of its elements.",
                    expected, found
                )
            }
            SemErrType::IncorrectReturnTy(expected, found) => {
                format!(
                    "Function expected return type {}, but found type {}.",
                    expected, found
                )
            }
            SemErrType::IncorrectArity(name, expected, found) => {
                format!(
                    "Function '{}' expected {} arguments, but found {}.",
                    name, expected, found
                )
            }

            SemErrType::UndefinedFunc(name) => format!("Function '{}' has not been defined.", name),
            SemErrType::UndefinedStruct(name) => format!("Struct '{}' has not been defined.", name),
            SemErrType::UndefinedVar(name) => {
                format!("Variable '{}' has not been defined in this scope.", name)
            }
            SemErrType::AlreadyDefinedVar(name) => {
                format!(
                    "Variable with name '{}' has already been defined in this scope.",
                    name
                )
            }
            SemErrType::AlreadyDefinedFunc(name) => {
                format!("Function with name '{}' has already been defined.", name)
            }
            SemErrType::AlreadyDefinedStruct(name) => {
                format!("Struct with name '{}' has already been defined.", name)
            }

            SemErrType::OpTypeMismatch(expected, op, found) => {
                format!(
                    "Operator '{}' Expects type '{}', but found type '{}'.",
                    op, expected, found
                )
            }
            SemErrType::VarDeclTypeMismatch(expected, found) => {
                format!(
                    "Variable was given type '{}', but found type '{}'.",
                    expected, found
                )
            }
            SemErrType::FieldTypeMismatch(expected, found) => {
                format!(
                    "Field was given type '{}', but found type '{}'.",
                    expected, found
                )
            }
            SemErrType::ArrElTypeMismatch(expected, found) => {
                format!(
                    "Not all elements in the array are of the same type. Array expected type '{}', but found type '{}'.",
                    expected, found
                )
            }
        };
        print_error(self.line, &msg);
    }
}
