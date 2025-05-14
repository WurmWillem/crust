use crate::error::print_error;
use std::collections::HashMap;

use crate::token::{Literal, Token, TokenType};

pub struct Scanner<'source> {
    source: &'source str,
    tokens: Vec<Token<'source>>,

    keywords: HashMap<String, TokenType>,

    start: usize,
    current: usize,
    line: u32,
    had_error: bool,
}

impl<'source> Scanner<'source> {
    pub fn new(source_file: &'source str) -> Self {
        macro_rules! create_keywords {
            ($($k: expr, $v: ident)*) => {
                HashMap::from([
                    $(($k.to_string(), TokenType::$v),)*
                ])
            };
        }

        let keywords = create_keywords!(
            "en",And "of",Or "if",If "else",Else "while",While "for",For
            "true",True "false",False "null",Null "dit",This "ouder",Super
            "klas",Class "fn",Fun "let",Var "return",Return "print",Print
            "int",F64 "bool",Bool "str",Str
        );

        let source_len = source_file.len();

        Self {
            source: source_file,
            tokens: Vec::with_capacity(source_len / 6),
            keywords,
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token<'source>>, ()> {
        while !self.at_end_input() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", Literal::None, self.line));

        if self.had_error {
            Err(())
        } else {
            Ok(self.tokens)
        }
    }

    fn at_end_input(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.get_current_char();
        self.current += 1;

        macro_rules! ternary {
            ($t1: ident, $t2: ident) => {{
                let token = if self.matches('=') {
                    self.current += 1;
                    TokenType::$t1
                } else {
                    TokenType::$t2
                };
                self.add_token(token);
            }};
        }

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            // '[' => self.add_token(TokenType::LeftBracket),
            // ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            // '^' => self.add_token(TokenType::Caret),
            '!' => ternary!(BangEqual, Bang),
            '=' => ternary!(EqualEqual, Equal),
            '<' => ternary!(LessEqual, Less),
            '>' => ternary!(GreaterEqual, Greater),
            '+' => ternary!(PlusEqual, Plus),

            // comments
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.at_end_input() {
                        self.current += 1;
                    }
                } else if self.matches('*') {
                    self.check_for_end_comment();
                } else {
                    self.add_token(TokenType::Slash);
                    self.current += 1;
                }
            }

            // strings
            '"' => {
                while self.peek() != '"' && !self.at_end_input() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.current += 1;
                }
                if self.at_end_input() {
                    print_error(self.line, "Ongetermineerde reeks.");
                    self.had_error = true;
                    return;
                }

                let str = &self.source[(self.start + 1)..self.current];
                self.add_lit_token(TokenType::String, Literal::Str(str));

                self.current += 1;
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            // keywords and variables
            _ => {
                if c.is_ascii_digit() {
                    self.add_num_token()
                } else if c.is_alphabetic() || c == '_' {
                    while self.peek().is_alphanumeric() || self.peek() == '_' {
                        self.current += 1;
                    }

                    // TODO: could be optimized with tries
                    let text = self.source[self.start..self.current].to_string();
                    let kind = match self.keywords.get(&text) {
                        Some(k) => *k,
                        None => TokenType::Identifier,
                    };

                    self.add_token(kind);
                } else {
                    let msg = format!("'{}' is een ongeldig karakter.", c);
                    print_error(self.line, &msg);
                    self.had_error = true;
                }
            }
        }
    }

    fn check_for_end_comment(&mut self) {
        while !self.at_end_input() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;

            if self.matches('/') && self.get_next_char() == '*' {
                self.current += 2;
                self.check_for_end_comment();
            }
            if self.matches('*') && self.get_next_char() == '/' {
                self.current += 2;
                return;
            }
        }
    }

    fn peek(&self) -> char {
        if self.at_end_input() {
            '\0'
        } else {
            self.get_current_char()
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        !self.at_end_input() && self.get_current_char() == expected
    }

    fn get_current_char(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }

    fn get_next_char(&self) -> char {
        self.source.as_bytes()[self.current + 1] as char
    }

    fn add_lit_token(&mut self, kind: TokenType, lit: Literal<'source>) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(kind, lexeme, lit, self.line));
    }

    fn add_token(&mut self, kind: TokenType) {
        self.add_lit_token(kind, Literal::None)
    }

    fn add_num_token(&mut self) {
        while self.peek().is_ascii_digit() {
            self.current += 1;
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.current += 1;

            while self.peek().is_ascii_digit() {
                self.current += 1;
            }
        }

        let num = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_lit_token(TokenType::Number, Literal::Num(num))
    }

    fn peek_next(&self) -> char {
        if self.current >= self.source.len() {
            '\0'
        } else {
            self.get_next_char()
        }
    }
}
