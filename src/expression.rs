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
    Var(&'a str),
    Call {
        name: &'a str,
        args: Vec<Expr<'a>>,
    },
    Assign {
        name: &'a str,
        value: Box<Expr<'a>>,
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
