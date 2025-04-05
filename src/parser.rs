use crate::{
    chunk::Chunk,
    error::print_error,
    opcode::OpCode,
    token::{Literal, Token, TokenType},
    value::StackValue,
};

struct ParseError {
    msg: String,
    line: usize,
}

pub struct Parser<'token> {
    tokens: Vec<Token<'token>>,
    chunk: Chunk,
    current: usize,
}
impl<'token> Parser<'token> {
    // fn new() -> Self {
    //     todo!()
    // }

    pub fn compile(self, tokens: Vec<Token>, chunk: Chunk) {
        let mut parser = Parser {
            tokens,
            chunk,
            current: 0,
        };

        parser.emit_byte(OpCode::Return as u8);
    }

    fn expression(&mut self) {}

    fn number(&mut self) {
        let Literal::Num(value) = self.peek().literal else {
            panic!("Unreachable.");
        };
        self.emit_constant(StackValue::F64(value));
    }

    fn emit_constant(&mut self, value: StackValue) {
        let const_index = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, const_index)
    }

    fn make_constant(&mut self, value: StackValue) -> u8 {
        let const_index = self.chunk.add_constant(value) as u8;
        if const_index > u8::MAX {
            // NOTE: maybe make this return a Result instead?
            print_error(self.peek().line, "Too many constants in one chunk.");
        }
        const_index
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                line: self.previous().line,
                msg: msg.to_string(),
            })
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        // TODO: update line
        let line = 0;
        self.chunk.write_byte_to_chunk(byte, line);
    }

    fn emit_bytes(&mut self, byte_0: u8, byte_1: u8) {
        self.emit_byte(byte_0);
        self.emit_byte(byte_1);
    }

    fn check(&mut self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
