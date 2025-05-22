use crate::{
    compiler_types::Precedence,
    token::{Literal, TokenType},
    value::ValueType,
    OpCode,
};

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
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
}
impl BinaryOp {
    pub fn get_precedency(self) -> Precedence {
        match self {
            BinaryOp::Add | BinaryOp::Sub => Precedence::Term,
            BinaryOp::Mul | BinaryOp::Div => Precedence::Factor,
            BinaryOp::Equal | BinaryOp::NotEqual => Precedence::Equality,
            BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                Precedence::Comparison
            }
            BinaryOp::And => Precedence::And,
            BinaryOp::Or => Precedence::Or,
        }
    }

    pub fn from_token_type(ty: TokenType) -> Self {
        match ty {
            TokenType::Plus => BinaryOp::Add,
            TokenType::Minus => BinaryOp::Sub,
            TokenType::Star => BinaryOp::Mul,
            TokenType::Slash => BinaryOp::Div,
            TokenType::EqualEqual => BinaryOp::Equal,
            TokenType::BangEqual => BinaryOp::NotEqual,
            TokenType::Less => BinaryOp::Less,
            TokenType::LessEqual => BinaryOp::LessEqual,
            TokenType::Greater => BinaryOp::Greater,
            TokenType::GreaterEqual => BinaryOp::GreaterEqual,
            TokenType::And => BinaryOp::And,
            TokenType::Or => BinaryOp::Or,
            _ => unreachable!(),
        }
    }

    pub fn to_op_code(self) -> OpCode {
        match self {
            BinaryOp::Add => OpCode::Add,
            BinaryOp::Sub => OpCode::Sub,
            BinaryOp::Mul => OpCode::Mul,
            BinaryOp::Div => OpCode::Div,
            BinaryOp::Equal => OpCode::Equal,
            BinaryOp::NotEqual => OpCode::NotEqual,
            BinaryOp::Less => OpCode::Less,
            BinaryOp::LessEqual => OpCode::LessEqual,
            BinaryOp::Greater => OpCode::Greater,
            BinaryOp::GreaterEqual => OpCode::GreaterEqual,
            BinaryOp::And => OpCode::And,
            BinaryOp::Or => OpCode::Or,
        }
    }
}
#[derive(Debug)]
pub struct Stmt<'a> {
    pub stmt: StmtType<'a>,
    pub line: u32,
}
impl<'a> Stmt<'a> {
    pub fn new(stmt: StmtType<'a>, line: u32) -> Stmt<'a> {
        Stmt { stmt, line }
    }
}

#[derive(Debug)]
pub enum StmtType<'a> {
    Expr(Expr<'a>),
    Var {
        name: &'a str,
        value: Expr<'a>,
        ty: ValueType,
    },
    Println(Expr<'a>),
    // TODO: maybe make this an expression
    Block(Vec<Stmt<'a>>),
    // rethink naming, boxing, and if "If" struct is necessary
    If {
        first_if: Box<If<'a>>,
        final_else: Option<Box<Stmt<'a>>>,
    },
    While {
        condition: Expr<'a>,
        body: Box<Stmt<'a>>,
    },
}

#[derive(Debug)]
pub struct If<'a> {
    pub condition: Expr<'a>,
    pub block: Stmt<'a>,
}
impl<'a> If<'a> {
    pub fn new(condition: Expr<'a>, block: Stmt<'a>) -> If<'a> {
        If { condition, block }
    }
}

#[derive(Debug)]
pub struct Expr<'a> {
    pub expr: ExprType<'a>,
    pub line: u32,
}
impl<'a> Expr<'a> {
    pub fn new(expr: ExprType<'a>, line: u32) -> Expr<'a> {
        Expr { expr, line }
    }
}

#[derive(Debug)]
pub enum ExprType<'a> {
    Lit(Literal<'a>),
    Var(&'a str),
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
