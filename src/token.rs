use crate::value::ValueType;

#[derive(Debug, Clone, Copy)]
pub enum Literal<'source> {
    None,
    Str(&'source str),
    Num(f64),
    True,
    False,
    Null,
}
impl<'source> Literal<'source> {
    pub fn as_value_type(self) -> ValueType {
        match self {
            Literal::None => unreachable!(),
            Literal::Str(_) => ValueType::Str,
            Literal::Num(_) => ValueType::Num,
            Literal::True | Literal::False => ValueType::Bool,
            Literal::Null => ValueType::Null,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'source> {
    pub kind: TokenType,
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
            kind,
            lexeme,
            literal,
            line,
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
    Colon,
    Semicolon,

    Minus,
    Plus,
    Slash,
    Star,

    // one or two character tokens
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
    String,
    Number,

    // keywords
    And,
    Struct,
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
    // Println,
    Return,
    Super,
    This,
    True,
    While,

    // var types
    F64,
    Bool,
    Str,

    // end of file
    Eof,
}
impl TokenType {
    // pub fn is_value_type(&self) -> bool {
    //     match self {
    //         TokenType::F64 | TokenType::Bool | TokenType::Str => true,
    //         _ => false,
    //     }
    // }
    pub fn as_value_type(&self) -> Option<ValueType> {
        match self {
            TokenType::F64 => Some(ValueType::Num),
            TokenType::Bool => Some(ValueType::Bool),
            TokenType::Str => Some(ValueType::Str),
            _ => None,
        }
    }
}
