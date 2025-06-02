use crate::{
    parse_types::BinaryOp,
    token::{Literal, TokenType},
};

#[derive(Debug, Clone)]
pub struct Expr<'a> {
    pub expr: ExprType<'a>,
    pub line: u32,
}
impl<'a> Expr<'a> {
    pub fn new(expr: ExprType<'a>, line: u32) -> Expr<'a> {
        Expr { expr, line }
    }
}

#[derive(Debug, Clone)]
pub enum ExprType<'a> {
    Lit(Literal<'a>),
    Array(Vec<Expr<'a>>),
    Var(&'a str),
    Call {
        name: &'a str,
        args: Vec<Expr<'a>>,
    },
    Dot {
        inst: Box<Expr<'a>>,
        property: &'a str,
    },
    DotResolved {
        inst: Box<Expr<'a>>,
        index: u8,
    },
    DotAssign {
        inst: Box<Expr<'a>>,
        property: &'a str,
        new_value: Box<Expr<'a>>,
    },
    DotAssignResolved {
        inst: Box<Expr<'a>>,
        index: u8,
        new_value: Box<Expr<'a>>,
    },
    Index {
        arr: Box<Expr<'a>>,
        index: Box<Expr<'a>>,
    },
    AssignIndex {
        arr: Box<Expr<'a>>,
        index: Box<Expr<'a>>,
        new_value: Box<Expr<'a>>,
    },
    Assign {
        name: &'a str,
        new_value: Box<Expr<'a>>,
    },
    Unary {
        prefix: TokenType,
        value: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        op: BinaryOp,
        right: Box<Expr<'a>>,
    },
}
