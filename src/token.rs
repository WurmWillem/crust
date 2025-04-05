#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    None,
    Str,
    Num(f64),
    True,
    False,
    Nil,
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'source> {
    pub kind: TokenType,
    lexeme: &'source str,
    pub literal: Literal,
    pub line: usize,
}
impl<'source> Token<'source> {
    pub fn new(kind: TokenType, lexeme: &'source str, literal: Literal, line: usize) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }

    // pub fn to_string(&self) -> String {
    //     //format!("{:?}{}{}", self.kind, self.lexeme, self.literal)
    //     match &self.literal {
    //         Literal::Str(s) => s.clone(),
    //         Literal::Num(n) => n.to_string(),
    //         _ => self.lexeme.clone(),
    //         /* _ => "".to_string(), */
    //     }
    //     //self.lexeme.clone()
    // }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    From,
    Until,
    Caret,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Println,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    //
    EOF,
}
