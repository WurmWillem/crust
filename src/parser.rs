use crate::{
    chunk::Chunk, opcode::OpCode, token::{Token, TokenType}
};

pub struct Parser<'token> {
    tokens: Vec<Token<'token>>,
    chunk: Chunk,
    current: usize,
}
impl<'token> Parser<'token> {
    // fn new() -> Self {
    //     todo!()
    // }

    pub fn compile(mut self, tokens: Vec<Token>, chunk: Chunk) {
        let mut parser = Parser {
            tokens,
            chunk,
            current: 0,
        };

        parser.emit_byte(OpCode::Return as u8);
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

    fn consume(&mut self, kind: TokenType) {}

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
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
