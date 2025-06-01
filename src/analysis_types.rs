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
    // TODO: check if ty is necessary
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
pub struct StructData<'a> {
    pub fields: Vec<(ValueType, &'a str)>,
}

pub type FuncHash<'a> = HashMap<&'a str, FuncData<'a>>;
pub type NatFuncHash<'a> = HashMap<&'a str, NatFuncData>;
pub type StructHash<'a> = HashMap<&'a str, StructData<'a>>;

pub fn get_nat_func_hash<'a>() -> HashMap<&'a str, NatFuncData> {
    let mut nat_funcs = HashMap::new();
    macro_rules! add_func {
        ($name: expr, $func: ident, $parameters: expr, $return_ty: expr) => {
            let nat_func = NatFuncData {
                parameters: $parameters,
                func: crate::native_funcs::$func,
                return_ty: $return_ty,
            };
            nat_funcs.insert($name, nat_func);
        };
    }

    use ValueType as VT;
    add_func!("clock", clock, vec![], VT::Num);
    add_func!("print", print, vec![VT::Any], VT::Null);
    add_func!("println", println, vec![VT::Any], VT::Null);
    add_func!("sin", sin, vec![VT::Num], VT::Num);
    add_func!("cos", cos, vec![VT::Num], VT::Num);
    add_func!("tan", tan, vec![VT::Num], VT::Num);
    add_func!("min", min, vec![VT::Num, VT::Num], VT::Num);
    add_func!("max", max, vec![VT::Num, VT::Num], VT::Num);
    add_func!("abs", abs, vec![VT::Num], VT::Num);
    add_func!("sqrt", sqrt, vec![VT::Num], VT::Num);
    add_func!("pow", pow, vec![VT::Num, VT::Num], VT::Num);
    add_func!("len", len, vec![VT::Arr(Box::new(VT::Any))], VT::Num);
    add_func!("print_heap", print_heap, vec![], VT::Null);

    nat_funcs
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
