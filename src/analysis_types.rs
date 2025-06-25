use std::collections::HashMap;

use crate::{
    error::{SemErr, SemErrType},
    object::NativeFunc,
    statement::Stmt,
    value::ValueType,
};

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    // binary
    Add,
    Sub,
    Mul,
    Div,

    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    And,
    Or,

    //unary
    Minus,
    Bang,
}
impl core::fmt::Display for Operator {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Equal => write!(f, "="),
            Operator::NotEqual => write!(f, "=="),
            Operator::Less => write!(f, "<"),
            Operator::LessEqual => write!(f, "<="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            Operator::Minus => write!(f, "-"),
            Operator::Bang => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncData<'a> {
    pub parameters: Vec<(ValueType, &'a str)>,
    pub body: Vec<Stmt<'a>>,
    pub return_ty: ValueType,
    pub line: u32,
    pub use_self: bool,
}
#[derive(Debug)]
pub struct NatFuncData {
    pub parameters: Vec<ValueType>,
    pub func: NativeFunc,
    pub return_ty: ValueType,
    pub use_self: bool,
}
#[derive(Debug)]
pub struct NatStructData<'a> {
    pub fields: Vec<(ValueType, &'a str)>,
    pub methods: Vec<(&'a str, NatFuncData)>,
}
impl<'a> NatStructData<'a> {
    pub fn get_method_data(
        &self,
        name: &str,
        property: &str,
        line: u32,
    ) -> Result<(u8, ValueType, bool, Vec<ValueType>), SemErr> {
        for (index, (method_name, data)) in self.methods.iter().enumerate() {
            if *method_name == property {
                let params = data.parameters.to_vec();
                return Ok((index as u8, data.return_ty.clone(), data.use_self, params));
            }
        }
        let ty = SemErrType::InvalidMethod(name.to_string(), property.to_string());
        Err(SemErr::new(line, ty))
    }
}
#[derive(Debug)]
pub struct StructData<'a> {
    pub fields: Vec<(ValueType, &'a str)>,
    pub methods: Vec<(&'a str, FuncData<'a>)>,
}
impl<'a> StructData<'a> {
    pub fn new(fields: Vec<(ValueType, &'a str)>) -> Self {
        Self {
            fields,
            methods: vec![],
        }
    }

    pub fn get_method_data(
        &self,
        name: &str,
        property: &str,
        line: u32,
    ) -> Result<(u8, ValueType, bool, Vec<ValueType>), SemErr> {
        for (index, (method_name, data)) in self.methods.iter().enumerate() {
            if *method_name == property {
                let params = data.parameters.iter().map(|p| p.0.clone()).collect();
                return Ok((index as u8, data.return_ty.clone(), data.use_self, params));
            }
        }
        let ty = SemErrType::InvalidMethod(name.to_string(), property.to_string());
        Err(SemErr::new(line, ty))
    }

    pub fn get_field_index(&self, name: String, property: &str, line: u32) -> Result<u8, SemErr> {
        let index = match self
            .fields
            .iter()
            .position(|(_, field_name)| *field_name == property)
        {
            Some(index) => index as u8,
            None => {
                let ty = SemErrType::InvalidPubField(name, property.to_string());
                return Err(SemErr::new(line, ty));
            }
        };
        Ok(index)
    }
}

pub struct EnityData<'a> {
    pub funcs: HashMap<&'a str, FuncData<'a>>,
    pub nat_funcs: HashMap<&'a str, Vec<NatFuncData>>,
    pub structs: HashMap<&'a str, StructData<'a>>,
    pub nat_structs: HashMap<&'a str, NatStructData<'a>>,
}
impl<'a> EnityData<'a> {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            nat_funcs: HashMap::new(),
            structs: HashMap::new(),
            nat_structs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol<'a> {
    name: &'a str,
    pub ty: ValueType,
}
impl<'a> Symbol<'a> {
    pub fn new(name: &'a str, ty: ValueType) -> Self {
        Self { name, ty }
    }
}

#[derive(Debug)]
pub struct SemanticScope<'a> {
    stack: Vec<HashMap<&'a str, Symbol<'a>>>,
}
impl<'a> SemanticScope<'a> {
    pub fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    pub fn begin_scope(&mut self) {
        self.stack.push(HashMap::new());
    }
    pub fn end_scope(&mut self) {
        self.stack.pop();
    }

    pub fn declare(&mut self, symbol: Symbol<'a>, line: u32) -> Result<(), SemErr> {
        let current = self.stack.last_mut().unwrap();
        if current.contains_key(symbol.name) {
            return Err(SemErr::new(
                line,
                SemErrType::AlreadyDefinedVar(symbol.name.to_string()),
            ));
        }
        current.insert(symbol.name, symbol);
        Ok(())
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol<'a>> {
        for scope in self.stack.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym.clone());
            }
        }
        None
    }
}
