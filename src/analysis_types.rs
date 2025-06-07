use std::collections::HashMap;

use crate::{
    error::{SemErrType, SemanticErr},
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

#[derive(Debug)]
pub struct FuncData<'a> {
    pub parameters: Vec<(ValueType, &'a str)>,
    pub body: Vec<Stmt<'a>>,
    pub return_ty: ValueType,
    pub line: u32,
}
#[derive(Debug)]
pub struct NatFuncData {
    pub parameters: Vec<ValueType>,
    pub func: NativeFunc,
    pub return_ty: ValueType,
}
#[derive(Debug)]
pub struct NatStructData<'a> {
    pub fields: Vec<(ValueType, &'a str)>,
    pub methods: Vec<(&'a str, NatFuncData)>,
}
impl<'a> NatStructData<'a> {
    pub fn get_method_index_and_return_ty(
        &self,
        name: &str,
        property: &str,
        line: u32,
    ) -> Result<(u8, ValueType, Vec<ValueType>), SemanticErr> {
        for (index, (method_name, data)) in self.methods.iter().enumerate() {
            if *method_name == property {
                let params = data.parameters.iter().map(|p| p.clone()).collect();
                return Ok((index as u8, data.return_ty.clone(), params));
            }
        }
        let ty = SemErrType::InvalidMethod(name.to_string(), property.to_string());
        Err(SemanticErr::new(line, ty))
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

    pub fn get_method_index_and_return_ty(
        &self,
        name: &str,
        property: &str,
        line: u32,
    ) -> Result<(u8, ValueType, Vec<ValueType>), SemanticErr> {
        for (index, (method_name, data)) in self.methods.iter().enumerate() {
            if *method_name == property {
                let params = data.parameters.iter().map(|p| p.0.clone()).collect();
                return Ok((index as u8, data.return_ty.clone(), params));
            }
        }
        let ty = SemErrType::InvalidMethod(name.to_string(), property.to_string());
        Err(SemanticErr::new(line, ty))
    }

    pub fn get_field_index(
        &self,
        name: String,
        property: &str,
        line: u32,
    ) -> Result<u8, SemanticErr> {
        let index = match self
            .fields
            .iter()
            .position(|(_, field_name)| *field_name == property)
        {
            Some(index) => index as u8,
            None => {
                let ty = SemErrType::InvalidPubField(name, property.to_string());
                return Err(SemanticErr::new(line, ty));
            }
        };
        Ok(index)
    }
}

pub type FuncHash<'a> = HashMap<&'a str, FuncData<'a>>;
pub type NatFuncHash<'a> = HashMap<&'a str, NatFuncData>;
pub type StructHash<'a> = HashMap<&'a str, StructData<'a>>;
pub type NatStructHash<'a> = HashMap<&'a str, NatStructData<'a>>;

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

    pub fn declare(&mut self, symbol: Symbol<'a>, line: u32) -> Result<(), SemanticErr> {
        let current = self.stack.last_mut().unwrap();
        if current.contains_key(symbol.name) {
            return Err(SemanticErr::new(
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
