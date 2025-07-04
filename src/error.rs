use colored::Colorize;

use crate::{analysis_types::Operator, value::ValueType};

pub const PRINT_TOKENS: bool = false;
pub const PRINT_PARSE_TREE: bool = false;
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
pub struct SemErr {
    ty: SemErrType,
    line: u32,
}
impl SemErr {
    pub fn new(line: u32, ty: SemErrType) -> Self {
        Self { ty, line }
    }
}
#[derive(Debug)]
pub enum SemErrType {
    NoMainFunc,
    InvalidInfix,
    InvalidPrefix,
    SelfOutsideStruct,
    SelfAsStaticStruct,
    InvalidStaticAccess,
    SelfInMethodWithoutSelfParam,
    UndefinedVar(String),
    FuncDefInFunc(String),
    UndefinedFunc(String),
    IndexNonArr(ValueType),
    StructDefInFunc(String),
    UndefinedStruct(String),
    AlreadyDefinedVar(String),
    AlreadyDefinedFunc(String),
    AlreadyDefinedEnum(String),
    AlreadyDefinedStruct(String),
    NatParamTypeMismatch(String),
    StaticMethodOnInstance(String),
    SelfOnStaticMethod,
    NoSelfOnMethod,
    InvalidTypeFieldAccess(ValueType),
    InvalidTypeMethodAccess(ValueType),
    NoReturnTy(String, ValueType),
    InvalidMethod(String, String),
    InvalidPubField(String, String),
    InvalidCast(ValueType, ValueType),
    IncorrectReturnTy(ValueType, ValueType),
    FieldTypeMismatch(ValueType, ValueType),
    ArrElTypeMismatch(ValueType, ValueType),
    VarDeclTypeMismatch(ValueType, ValueType),
    AssignArrTypeMismatch(ValueType, ValueType),
    IncorrectArity(String, u8, u8),
    OpTypeMismatch(ValueType, Operator, ValueType),
    ParamTypeMismatch(String, ValueType, ValueType),
}
impl SemErr {
    pub fn print(&self) {
        //dbg!(&self.ty);
        let msg = match &self.ty {
            SemErrType::InvalidPrefix => "invalid prefix.".to_string(),
            SemErrType::InvalidInfix => "invalid infix.".to_string(),
            SemErrType::InvalidStaticAccess => "You can only use the '::' syntax for static methods.".to_string(),
            SemErrType::FuncDefInFunc(name) => format!("You attempted to define the function '{}' inside another function, which is illegal.", name.green()),
            SemErrType::StructDefInFunc(name) => format!("You attempted to define the struct '{}' inside a function, which is illegal.", name.green()),
            SemErrType::InvalidCast(expected, found) => format!("You can't cast an expression of type '{}' to type '{}'.", found, expected),
            SemErrType::NoMainFunc => {
                "You have to define a function with the name 'main' as entry point for the program."
                    .to_string()
            }
            SemErrType::SelfOutsideStruct => {
                "'self.property' can only be used inside methods of structs.".to_string()
            }
            SemErrType::SelfInMethodWithoutSelfParam => {
                "'self.property' can only be used inside methods with 'self' as parameter.".to_string()
            }
            SemErrType::SelfAsStaticStruct => {
                "'self::property' is invalid syntax as self is not static. Did you mean 'self.property'?".to_string()
            }
            SemErrType::StaticMethodOnInstance(inst_name) => format!("You cannot use a static method on an instance ({}).", inst_name.green()),
            SemErrType::SelfOnStaticMethod => "'struct::property' can only be used for static methods which don't have self as parameter.".to_string(),
            SemErrType::NoSelfOnMethod => "'instance.property' can only be used for non-static methods which have self as parameter.".to_string(),
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
            SemErrType::AlreadyDefinedEnum(name) => {
                format!(
                    "Enum with name '{}' has already been defined.",
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
            SemErrType::ParamTypeMismatch(name, expected, found) => {
                format!(
                    "Parameter of function '{}' has type '{}', but found type '{}'.",
                    name, expected, found
                )
            }
            SemErrType::NatParamTypeMismatch(name) => {
                format!(
                    "The types of the parameters of function '{}' and the types of the given arguments don't match.",
                    name.green()
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
