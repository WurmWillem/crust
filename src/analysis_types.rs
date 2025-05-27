use std::collections::HashMap;

use crate::{
    error::{ErrType, SemanticErr},
    statement::{Stmt, StmtType},
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

pub struct FuncData<'a> {
    pub parameters: Vec<(ValueType, &'a str)>,
    pub body: Vec<Stmt<'a>>,
    pub return_ty: ValueType,
    pub line: u32,
}

pub fn get_func_data<'a>(stmts: &Vec<Stmt<'a>>) -> HashMap<&'a str, FuncData<'a>> {
    let mut funcs = HashMap::new();
    for stmt in stmts {
        if let StmtType::Func {
            name,
            parameters,
            body,
            return_ty,
        } = &stmt.stmt
        {
            let func_data = FuncData {
                parameters: parameters.clone(),
                body: body.clone(),
                return_ty: *return_ty,
                line: stmt.line,
            };
            // WARN: add error handling
            funcs.insert(*name, func_data);
        }
    }
    funcs
}

#[derive(Debug, Clone, Copy)]
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
                ErrType::AlreadyDefinedVar(symbol.name.to_string()),
            ));
        }
        current.insert(symbol.name, symbol);
        Ok(())
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol<'a>> {
        for scope in self.stack.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(*sym);
            }
        }
        None
    }
}
