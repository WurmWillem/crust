#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal<'source> {
    None,
    Str(&'source str),
    Num(f64),
    // other types of literals such as true, false, or null are not necessary
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token<'source> {
    pub kind: TokenType,
    pub lexeme: &'source str,
    pub literal: Literal<'source>,
    pub line: u32,
}
impl<'source> Token<'source> {
    pub fn new(kind: TokenType, lexeme: &'source str, literal: Literal<'source>, line: u32) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
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

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Null,
    Or,
    Print,
    // Println,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // end of file
    Eof,
}
