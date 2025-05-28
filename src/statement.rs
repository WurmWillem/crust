use crate::{expression::Expr, value::ValueType};

#[derive(Debug, Clone)]
pub struct Stmt<'a> {
    pub stmt: StmtType<'a>,
    pub line: u32,
}
impl<'a> Stmt<'a> {
    pub fn new(stmt: StmtType<'a>, line: u32) -> Stmt<'a> {
        Stmt { stmt, line }
    }
}

#[derive(Debug, Clone)]
pub enum StmtType<'a> {
    Expr(Expr<'a>),
    Var {
        name: &'a str,
        value: Expr<'a>,
        ty: ValueType,
    },
    Println(Expr<'a>),
    Return(Expr<'a>),
    Break,
    Continue,
    Block(Vec<Stmt<'a>>),
    If {
        condition: Expr<'a>,
        body: Box<Stmt<'a>>,
        final_else: Option<Box<Stmt<'a>>>,
    },
    While {
        condition: Expr<'a>,
        body: Box<Stmt<'a>>,
    },
    For {
        var: Box<Stmt<'a>>,
        condition: Expr<'a>,
        body: Box<Stmt<'a>>,
    },
    Func {
        name: &'a str,
        parameters: Vec<(ValueType, &'a str)>,
        body: Vec<Stmt<'a>>,
        return_ty: ValueType,
    },
}
