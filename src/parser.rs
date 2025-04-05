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
impl ParseError {
   fn new(line: usize, msg: String) -> Self {
       Self { msg, line }
    } 
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
enum Precedence {
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
// impl Precedence {
//    #[inline(always)]
//     fn rule(self) -> &'static ParseRule {
//         &PARSE_RULES[self as usize]
//     }
// }

type ParseFn = for<'parser> fn(&mut Parser<'parser>) -> Result<(), ParseError>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum FnType {
    Grouping,
    Unary,
    Binary,
    Number,
    Empty,
}

#[derive(Clone, Copy)]
struct ParseRule {
    prefix: FnType,
    infix: FnType,
    precedence: Precedence,
}

#[rustfmt::skip]
const PARSE_RULES: [ParseRule; 39] = {
    use FnType::*;
    use Precedence as P;

    macro_rules! none {
        () => {
            ParseRule { prefix: Empty, infix: Empty, precedence: P::None }
        }
    }

    [
        // left paren
        ParseRule { prefix: Grouping, infix: Empty, precedence: P::None, },
        none!(), // right paren
        none!(), // left brace
        none!(), // right brace
        none!(), // comma
        none!(), // dot
        // minus
        ParseRule { prefix: Unary, infix: Binary, precedence: P::Term, },
        // plus
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Term, },
                 //
        none!(), // semicolon
        // slash
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Factor, },
        // star
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Factor, },
        none!(), // bang
        none!(), // bang equal
        none!(), // equal
        none!(), // equal equal
        none!(), // greater
        none!(), // greater equal
        none!(), // less
        none!(), // less equal
        none!(), // identifier
        none!(), // string
        // number
        ParseRule { prefix: Number, infix: Empty, precedence: P::None, },
        none!(), // and
        none!(), // class
        none!(), // else
        none!(), // false
        none!(), // for
        none!(), // fun
        none!(), // if
        none!(), // nil
        none!(), // or
        none!(), // print
        none!(), // return
        none!(), // super
        none!(), // this
        none!(), // true
        none!(), // var
        none!(), // while
        none!(), // EOF
    ]
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

    pub fn compile(tokens: Vec<Token>, chunk: Chunk) -> Chunk {
        let mut parser = Parser {
            tokens,
            chunk,
            current: 0,
        };

        // parser.advance();
        parser.expression();

        parser.emit_byte(OpCode::Return as u8);
        parser.chunk.disassemble("code");
        parser.chunk
    }

    fn execute_fn_type(&mut self, fn_type: FnType) -> Result<(), ParseError> {
        match fn_type {
            FnType::Grouping => self.grouping(),
            FnType::Unary => self.unary(),
            FnType::Binary => self.binary(),
            FnType::Number => self.number(),
            FnType::Empty => Ok(()),
        }
    }

    fn get_rule(&mut self, kind: TokenType) -> ParseRule {
        PARSE_RULES[kind as usize]
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), ParseError> {
        self.advance();
        let kind = self.previous().kind;
        // dbg!(kind);
        let prefix = self.get_rule(kind).prefix;
        // dbg!(prefix);

        if prefix == FnType::Empty {
            let msg = "Expected expression.".to_string();
            let err = ParseError::new(self.peek().line, msg);
            return Err(err);
        }
        self.execute_fn_type(prefix)?;
        // dbg!(prefix);

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
        self.emit_constant(StackValue::F64(value));
        Ok(())
    }

    fn binary(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn unary(&mut self) -> Result<(), ParseError> {
        let operator_type = self.previous().kind;

        self.parse_precedence(Precedence::Unary)?;

        match operator_type {
            TokenType::Minus => {
                self.emit_byte(OpCode::Negate as u8);
                dbg!("ey");
                // TODO: make it crash if - is applied to non-number
            }
            _ => panic!("Unreachable."),
        }
        Ok(())
    }

    fn grouping(&mut self) -> Result<(), ParseError> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after expression.")
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
