use crate::{
    chunk::Chunk,
    opcode::OpCode,
    parse_helpers::*,
    token::{Literal, Token, TokenType},
    value::StackValue,
};

pub struct Parser<'token> {
    tokens: Vec<Token<'token>>,
    chunk: Chunk,
    current: usize,
}
impl<'token> Parser<'token> {
    pub fn compile(tokens: Vec<Token>, chunk: Chunk) -> Result<Chunk, ParseError> {
        let mut parser = Parser {
            tokens,
            chunk,
            current: 0,
        };

        parser.expression()?;

        parser.emit_byte(OpCode::Return as u8);
        // parser.chunk.disassemble("code");
        Ok(parser.chunk)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), ParseError> {
        self.advance();
        let kind = self.previous().kind;
        // dbg!(kind);
        let prefix = self.get_rule(kind).prefix;
        // dbg!(prefix);

        if prefix == FnType::Empty {
            let msg = "Expected expression.";
            let err = ParseError::new(self.peek().line, msg);
            return Err(err);
        }
        self.execute_fn_type(prefix)?;
        // dbg!(self.peek().kind);
        // let operand_type = ./

        while precedence <= self.get_rule(self.peek().kind).precedence {
            self.advance();
            let infix = self.get_rule(self.previous().kind).infix;
            self.execute_fn_type(infix)?;
        }
        Ok(())
    }

    fn expression(&mut self) -> Result<(), ParseError> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn number(&mut self) -> Result<(), ParseError> {
        let Literal::Num(value) = self.previous().literal else {
            panic!("Unreachable.");
        };
        self.emit_constant(StackValue::F64(value))
    }

    fn binary(&mut self) -> Result<(), ParseError> {
        let op_type = self.previous().kind;
        let rule = self.get_rule(op_type);

        let new_precedence = (rule.precedence as u8 + 1).into();
        self.parse_precedence(new_precedence)?;

        // TODO: return error if operator applied to non-number

        match op_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Sub as u8),
            TokenType::Star => self.emit_byte(OpCode::Mul as u8),
            TokenType::Slash => self.emit_byte(OpCode::Div as u8),
            TokenType::EqualEqual => self.emit_byte(OpCode::Equal as u8),
            TokenType::Greater => self.emit_byte(OpCode::Greater as u8),
            TokenType::GreaterEqual => self.emit_byte(OpCode::GreaterEqual as u8),
            TokenType::Less => self.emit_byte(OpCode::Less as u8),
            TokenType::LessEqual => self.emit_byte(OpCode::LessEqual as u8),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn unary(&mut self) -> Result<(), ParseError> {
        let operator_type = self.previous().kind;
        let operand_type = self.peek().kind;

        self.parse_precedence(Precedence::Unary)?;

        match operator_type {
            // TODO: make them stackable
            TokenType::Minus => {
                if operand_type != TokenType::Number {
                    let msg = "'-' can only be applied to numbers.";
                    return Err(ParseError::new(self.peek().line, msg));
                }
                self.emit_byte(OpCode::Negate as u8);
            }
            TokenType::Bang => {
                if operand_type != TokenType::True && operand_type != TokenType::False {
                    let msg = "'!' can only be applied to booleans.";
                    return Err(ParseError::new(self.peek().line, msg));
                }
                self.emit_byte(OpCode::Not as u8);
            }
            _ => unreachable!("Unreachable."),
        }
        Ok(())
    }

    fn grouping(&mut self) -> Result<(), ParseError> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after expression.")
    }

    fn literal(&mut self) -> Result<(), ParseError> {
        match self.previous().kind {
            TokenType::True => self.emit_byte(OpCode::True as u8),    
            TokenType::False => self.emit_byte(OpCode::False as u8),    
            TokenType::Null => self.emit_byte(OpCode::Null as u8),    
            _ => unreachable!(),
        }
        Ok(())
    }

    fn execute_fn_type(&mut self, fn_type: FnType) -> Result<(), ParseError> {
        match fn_type {
            FnType::Grouping => self.grouping(),
            FnType::Unary => self.unary(),
            FnType::Binary => self.binary(),
            FnType::Number => self.number(),
            FnType::Literal => self.literal(),
            FnType::Empty => Ok(()),
        }
    }

    fn emit_constant(&mut self, value: StackValue) -> Result<(), ParseError> {
        let const_index = self.make_constant(value)?;
        self.emit_bytes(OpCode::Constant as u8, const_index);
        Ok(())
    }

    fn make_constant(&mut self, value: StackValue) -> Result<u8, ParseError> {
        let const_index = self.chunk.add_constant(value);
        if const_index > u8::MAX.into() {
            let msg = "Too many constants in one chunk.";
            return Err(ParseError::new(self.peek().line, msg));
        }
        Ok(const_index as u8)
    }

    fn get_rule(&mut self, kind: TokenType) -> ParseRule {
        PARSE_RULES[kind as usize]
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), ParseError> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                line: self.previous().line,
                msg: msg.to_string(),
            })
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte_to_chunk(byte, self.peek().line);
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
        self.peek().kind == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
