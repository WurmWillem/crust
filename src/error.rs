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
    InvalidThis,
    NoMainFunc,
    FuncDefInFunc(String),
    StructDefInFunc(String),
    InvalidTypeMethodAccess(ValueType),
    InvalidTypeFieldAccess(ValueType),
    InvalidPubField(String, String),
    InvalidMethod(String, String),
    IndexNonArr(ValueType),
    AssignArrTypeMismatch(ValueType, ValueType),
    UndefinedFunc(String),
    UndefinedStruct(String),
    IncorrectArity(String, u8, u8),
    IncorrectReturnTy(ValueType, ValueType),
    NoReturnTy(String, ValueType),
    UndefinedVar(String),
    AlreadyDefinedVar(String),
    AlreadyDefinedFunc(String),
    AlreadyDefinedStruct(String),
    OpTypeMismatch(ValueType, Operator, ValueType),
    VarDeclTypeMismatch(ValueType, ValueType),
    ParamTypeMismatch(ValueType, ValueType),
    FieldTypeMismatch(ValueType, ValueType),
    ArrElTypeMismatch(ValueType, ValueType),
}
impl SemanticErr {
    pub fn print(&self) {
        //dbg!(&self.ty);
        let msg = match &self.ty {
            SemErrType::InvalidPrefix => "invalid prefix.".to_string(),
            SemErrType::InvalidInfix => "invalid infix.".to_string(),
            SemErrType::FuncDefInFunc(name) => format!("You attempted to define the function '{}' inside another function, which is illegal.", name.green()),
            SemErrType::StructDefInFunc(name) => format!("You attempted to define the struct '{}' inside a function, which is illegal.", name.green()),
            SemErrType::NoMainFunc => {
                "You have to define a function with the name 'main' as entry point for the program."
                    .to_string()
            }
            SemErrType::InvalidThis => {
                "'self' can only be used inside methods of structs.".to_string()
            }
            SemErrType::InvalidTypeMethodAccess(ty) => {
                format!(
                    "You can only access methods of instances, but you tried to access a method of type '{}'.",
                    ty
                )
            }
            SemErrType::InvalidTypeFieldAccess(ty) => {
                format!(
                    "You can only access fields of instances, but you tried to access a field of type '{}'.",
                    ty
                )
            }
            SemErrType::InvalidPubField(name, property) => {
                format!("Struct '{}' has no field named '{}'.", name, property)
            }
            SemErrType::InvalidMethod(name, property) => {
                format!("Struct '{}' has no method named '{}'.", name, property)
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
            SemErrType::NoReturnTy(name, return_ty) => {
                format!(
                    "Function '{}' has return type '{}', but no return statement was found.",
                    name, return_ty
                )
            }
            SemErrType::IncorrectArity(name, expected, found) => {
                format!(
                    "Function '{}' expected {} argument(s), but found {}.",
                    name.green(),
                    expected,
                    found
                )
            }

            SemErrType::UndefinedFunc(name) => {
                format!("Function '{}' has not been defined.", name.green())
            }
            SemErrType::UndefinedStruct(name) => {
                format!("Struct '{}' has not been defined.", name.green())
            }
            SemErrType::UndefinedVar(name) => {
                format!(
                    "Variable '{}' has not been defined in this scope.",
                    name.green()
                )
            }
            SemErrType::AlreadyDefinedVar(name) => {
                format!(
                    "Variable with name '{}' has already been defined in this scope.",
                    name.green()
                )
            }
            SemErrType::AlreadyDefinedFunc(name) => {
                format!(
                    "Function with name '{}' has already been defined.",
                    name.green()
                )
            }
            SemErrType::AlreadyDefinedStruct(name) => {
                format!(
                    "Struct with name '{}' has already been defined.",
                    name.green()
                )
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
            SemErrType::ParamTypeMismatch(expected, found) => {
                format!(
                    "Parameter has type '{}', but found type '{}'.",
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
