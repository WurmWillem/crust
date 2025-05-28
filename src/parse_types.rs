use crate::{analysis_types::Operator, token::TokenType, OpCode};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}
impl std::convert::From<u8> for Precedence {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Assignment,
            2 => Self::Or,
            3 => Self::And,
            4 => Self::Equality,
            5 => Self::Comparison,
            6 => Self::Term,
            7 => Self::Factor,
            8 => Self::Unary,
            9 => Self::Call,
            10 => Self::Primary,
            _ => panic!("Not a valid value for Precedence."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum FnType {
    Empty,
    Grouping,
    Unary,
    Binary,
    Number,
    String,
    Literal,
    Var,
    Call,
}

#[derive(Clone, Copy)]
pub struct ParseRule {
    pub prefix: FnType, // stores in what way can it be used as prefix (if used at all)
    pub infix: FnType,
    pub precedence: Precedence,
}

#[rustfmt::skip]
pub const PARSE_RULES: [ParseRule; 46] = {
    use FnType::*;
    use Precedence as P;

    macro_rules! none {
        () => {
            ParseRule { prefix: Empty, infix: Empty, precedence: P::None }
        }
    }

    [
        
        ParseRule { prefix: Grouping, infix: Call, precedence: P::Call, }, // left paren
        none!(), // right paren
        none!(), // left brace
        none!(), // right brace
        none!(), // comma
        none!(), // dot
        none!(), // colon
        none!(), // semicolon
        ParseRule { prefix: Unary, infix: Binary, precedence: P::Term, }, // minus
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Term, }, // plus
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Factor, }, // slash
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Factor, }, // star
        ParseRule { prefix: Unary, infix: Empty, precedence: P::Factor, }, // bang
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // bang equal
        none!(), // equal
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // equal equal
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // Greater
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // Greater equal
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // Less
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // Less equal

        none!(), //Plus Equal
        none!(), //Minus Equal
        none!(), //Mul Equal
        none!(), //Div Equal

        ParseRule { prefix: Var, infix: Empty, precedence: P::None, }, // identifier
        ParseRule { prefix: String, infix: Empty, precedence: P::None, }, // string
        ParseRule { prefix: Number, infix: Empty, precedence: P::None, }, // number
        ParseRule { prefix: Empty, infix: Binary, precedence: P::And, }, // and
        none!(), // class
        none!(), // else
        ParseRule { prefix: Literal, infix: Empty, precedence: P::None, }, // false
        none!(), // for
        none!(), // break
        none!(), // in
        none!(), // to
        none!(), // fun
        none!(), // if
        ParseRule { prefix: Literal, infix: Empty, precedence: P::None, }, // nil
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Or, }, // or
        none!(), // print
        none!(), // return
        none!(), // super
        none!(), // this
        ParseRule { prefix: Literal, infix: Empty, precedence: P::None, }, // true
        none!(), // while
        none!(), // EOF
    ]
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

    pub fn to_operator(self) -> Operator {
        match self {
            BinaryOp::Add => Operator::Add,
            BinaryOp::Sub => Operator::Sub,
            BinaryOp::Mul => Operator::Mul,
            BinaryOp::Div => Operator::Div,
            BinaryOp::Equal => Operator::Equal,
            BinaryOp::NotEqual => Operator::NotEqual,
            BinaryOp::Less => Operator::Less,
            BinaryOp::LessEqual => Operator::LessEqual,
            BinaryOp::Greater => Operator::Greater,
            BinaryOp::GreaterEqual => Operator::GreaterEqual,
            BinaryOp::And => Operator::And,
            BinaryOp::Or => Operator::Or,
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
