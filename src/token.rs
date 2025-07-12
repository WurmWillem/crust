use crate::{parse_types::ParseRule, value::ValueType};

#[derive(Debug, Clone, Copy)]
pub enum Literal<'source> {
    None,
    Str(&'source str),
    F64(f64),
    I64(i64),
    U64(u64),
    True,
    False,
    Null,
}
impl<'source> Literal<'source> {
    pub fn as_value_type(self) -> ValueType {
        match self {
            Literal::None => unreachable!(),
            Literal::Str(_) => ValueType::Str,
            Literal::F64(_) => ValueType::F64,
            Literal::U64(_) => ValueType::U64,
            Literal::I64(_) => ValueType::I64,
            Literal::True | Literal::False => ValueType::Bool,
            Literal::Null => ValueType::Null,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'source> {
    pub ty: TokenType,
    pub lexeme: &'source str,
    pub literal: Literal<'source>,
    pub line: u32,
}
impl<'source> Token<'source> {
    pub fn new(
        kind: TokenType,
        lexeme: &'source str,
        literal: Literal<'source>,
        line: u32,
    ) -> Self {
        Self {
            ty: kind,
            lexeme,
            literal,
            line,
        }
    }
    pub fn as_value_type(&self) -> Option<ValueType> {
        match self.ty {
            TokenType::F64 => Some(ValueType::F64),
            TokenType::I64 => Some(ValueType::I64),
            TokenType::U64 => Some(ValueType::U64),
            TokenType::Bool => Some(ValueType::Bool),
            TokenType::Str => Some(ValueType::Str),
            TokenType::Identifier => Some(ValueType::UnknownType(self.lexeme.to_string())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Comma,
    Dot,
    Semicolon,

    Minus,
    Plus,
    Slash,
    Star,

    // one or two character tokens
    Colon,
    DoubleColon,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    PlusEqual,
    MinEqual,
    MulEqual,
    DivEqual,

    // literals
    Identifier,
    StringLit,
    Num,

    // keywords
    As,
    And,
    Struct,
    Enum,
    Else,
    False,
    For,
    Break,
    Continue,
    In,
    To,
    Fn,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    While,

    // var decl
    F64,
    I64,
    U64,
    Bool,
    Str,

    // end of file
    Eof,
}
impl TokenType {
    pub fn to_parse_rule(self) -> ParseRule {
        use crate::parse_types::FnType as F;
        use crate::parse_types::Precedence as P;
        use TokenType as TT;

        match self {
            TT::LeftParen => ParseRule::new(F::Grouping, F::Call, P::Call),
            TT::LeftBracket => ParseRule::new(F::Array, F::Index, P::Call),
            TT::Dot => ParseRule::new(F::Empty, F::Dot, P::Call),
            TT::DoubleColon => ParseRule::new(F::Empty, F::DoubleColon, P::Call),
            TT::Minus => ParseRule::new(F::Unary, F::Binary, P::Term),
            TT::Plus => ParseRule::new(F::Empty, F::Binary, P::Term),
            TT::Slash | TT::Star => ParseRule::new(F::Empty, F::Binary, P::Factor),
            TT::Bang => ParseRule::new(F::Unary, F::Empty, P::Factor),
            TT::BangEqual | TT::Greater | TT::GreaterEqual | TT::Less | TT::LessEqual => {
                ParseRule::new(F::Empty, F::Binary, P::Comparison)
            }
            TT::Identifier => ParseRule::new(F::Var, F::Empty, P::None),
            TT::StringLit => ParseRule::new(F::String, F::Empty, P::None),
            TT::Num => ParseRule::new(F::Number, F::Empty, P::None),
            TT::As => ParseRule::new(F::Number, F::Cast, P::Call),
            TT::And => ParseRule::new(F::Empty, F::Binary, P::And),
            TT::Or => ParseRule::new(F::Empty, F::Binary, P::Or),
            TT::False | TT::True | TT::Null => ParseRule::new(F::Literal, F::Empty, P::None),
            TT::This => ParseRule::new(F::This, F::Empty, P::None),
            _ => ParseRule::new(F::Empty, F::Empty, P::None),
        }
    }
    // pub fn is_value_type(&self) -> bool {
    //     match self {
    //         TokenType::F64 | TokenType::Bool | TokenType::Str => true,
    //         _ => false,
    //     }
    // }
}
