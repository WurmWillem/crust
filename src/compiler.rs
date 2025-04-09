use colored::Colorize;

use crate::{
    chunk::Chunk,
    compiler_helper::*,
    error::{print_error, ParseError},
    object::{Object, ObjectValue},
    opcode::OpCode,
    token::{Literal, Token, TokenType},
    value::{StackValue, ValueType},
};

pub struct Compiler<'token> {
    tokens: Vec<Token<'token>>,
    objects: Vec<Object>,
    chunk: Chunk,
    current: usize,
    last_operand_type: ValueType,
}
impl<'token> Compiler<'token> {
    pub fn compile(tokens: Vec<Token>, chunk: Chunk) -> Option<(Chunk, Vec<Object>)> {
        let mut compiler = Compiler {
            tokens,
            chunk,
            current: 0,
            last_operand_type: ValueType::None,
            objects: Vec::new(),
        };

        let mut had_error = false;
        while !compiler.matches(TokenType::Eof) {
            if let Err(err) = compiler.declaration() {
                print_error(err.line, &err.msg);

                had_error = true;
                compiler.synchronize();
            }
        }
        if had_error {
            println!("{}", "Parse error(s) detected, terminate program.".red());
            return None;
        }

        if compiler.current != compiler.tokens.len() - 1 {
            println!("{}", "Not all tokens were parsed.".red());
        }
        // compiler.chunk.disassemble("code");

        compiler.emit_byte(OpCode::Return as u8);
        Some((compiler.chunk, compiler.objects))
    }

    fn declaration(&mut self) -> Result<(), ParseError> {
        if self.matches(TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }
    fn var_declaration(&mut self) -> Result<(), ParseError> {
        let global = self.parse_var()?;

        if self.matches(TokenType::Equal) {
            self.expression()?;
        } else {
            self.emit_byte(OpCode::Null as u8);
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        self.emit_bytes(OpCode::DefineGlobal as u8, global);
        Ok(())
    }

    fn statement(&mut self) -> Result<(), ParseError> {
        // dbg!(self.peek());
        if self.matches(TokenType::Print) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<(), ParseError> {
        // dbg!(self.chunk.constants.len());
        self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after statement.")?;
        self.emit_byte(OpCode::Print as u8);
        // dbg!(self.chunk.constants.len());
        Ok(())
    }

    fn expression_statement(&mut self) -> Result<(), ParseError> {
        self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' afer expression.")?;
        self.emit_byte(OpCode::Pop as u8);
        Ok(())
    }

    fn synchronize(&mut self) {
        while self.peek().kind != TokenType::Eof {
            if self.previous().kind == TokenType::Semicolon {
                return;
            }
            self.advance();
        }
    }

    fn parse_var(&mut self) -> Result<u8, ParseError> {
        self.consume(TokenType::Identifier, "Expected variable name.")?;

        let var_token = self.previous().lexeme.to_string();
        self.identifier_constant(var_token)
    }

    fn identifier_constant(&mut self, lexeme: String) -> Result<u8, ParseError> {
        let idx = self.objects.len();

        let var_name = Object {
            value: ObjectValue::Str(lexeme),
        };
        self.objects.push(var_name);

        self.make_constant(StackValue::Obj(idx))
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
        let can_assign = precedence <= Precedence::Assignment;
        self.execute_fn_type(prefix, can_assign)?;

        while precedence <= self.get_rule(self.peek().kind).precedence {
            self.advance();
            let infix = self.get_rule(self.previous().kind).infix;
            // dbg!(infix);
            self.execute_fn_type(infix, can_assign)?;
        }
        Ok(())
    }

    fn expression(&mut self) -> Result<(), ParseError> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn string(&mut self) -> Result<(), ParseError> {
        let Literal::Str(value) = self.previous().literal else {
            unreachable!();
        };
        let obj = Object {
            value: ObjectValue::Str(value.to_string()),
        };

        self.objects.push(obj);
        self.last_operand_type = ValueType::Str;

        let len = self.objects.len() - 1;
        self.emit_constant(StackValue::Obj(len))
    }

    fn number(&mut self) -> Result<(), ParseError> {
        let Literal::Num(value) = self.previous().literal else {
            unreachable!();
        };
        self.last_operand_type = ValueType::Num;
        self.emit_constant(StackValue::F64(value))
    }

    fn binary(&mut self) -> Result<(), ParseError> {
        let op_type = self.previous().kind;
        let rule = self.get_rule(op_type);

        let lhs_type = self.last_operand_type;

        let new_precedence = (rule.precedence as u8 + 1).into();
        self.parse_precedence(new_precedence)?;

        macro_rules! emit_op_code {
            ($op_char: expr, $op_code: ident) => {{
                if lhs_type != ValueType::Num || self.last_operand_type != ValueType::Num {
                    let msg = &format!("{} can only be applied to numbers.", $op_char);
                    return Err(ParseError::new(self.peek().line, msg));
                }
                self.emit_byte(OpCode::$op_code as u8);
            }};
        }
        macro_rules! emit_and_update_last_operand {
            ($op_char: expr, $op_code: ident) => {{
                emit_op_code!($op_char, $op_code);
                self.last_operand_type = ValueType::Bool;
            }};
        }

        match op_type {
            TokenType::Plus => {
                if lhs_type != self.last_operand_type
                    || (lhs_type != ValueType::Num && lhs_type != ValueType::Str)
                {
                    let msg = "'+' can only be applied to numbers and strings.";
                    return Err(ParseError::new(self.peek().line, msg));
                }
                self.emit_byte(OpCode::Add as u8);
            }
            TokenType::Minus => emit_op_code!('-', Sub),
            TokenType::Star => emit_op_code!('*', Mul),
            TokenType::Slash => emit_op_code!('/', Div),
            TokenType::EqualEqual => {
                if lhs_type != self.last_operand_type
                    || (lhs_type != ValueType::Num
                        && lhs_type != ValueType::Bool
                        && lhs_type != ValueType::Null)
                {
                    let msg = "'==' can only be applied to 2 numbers, bools, or nulls";
                    return Err(ParseError::new(self.peek().line, msg));
                }
                self.emit_byte(OpCode::Equal as u8);
                self.last_operand_type = ValueType::Bool;
            }
            TokenType::BangEqual => emit_and_update_last_operand!("!=", BangEqual),
            TokenType::Greater => emit_and_update_last_operand!('>', Greater),
            TokenType::GreaterEqual => emit_and_update_last_operand!(">=", GreaterEqual),
            TokenType::Less => emit_and_update_last_operand!('<', Less),
            TokenType::LessEqual => emit_and_update_last_operand!("<=", LessEqual),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn unary(&mut self) -> Result<(), ParseError> {
        let operator_type = self.previous().kind;

        self.parse_precedence(Precedence::Unary)?;

        match operator_type {
            TokenType::Minus => {
                if self.last_operand_type != ValueType::Num {
                    let msg = "'-' can only be applied to numbers.";
                    return Err(ParseError::new(self.peek().line, msg));
                }
                self.emit_byte(OpCode::Negate as u8);
            }
            TokenType::Bang => {
                if self.last_operand_type != ValueType::Bool {
                    let msg = "'!' can only be applied to booleans.";
                    return Err(ParseError::new(self.peek().line, msg));
                }
                self.emit_byte(OpCode::Not as u8);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn grouping(&mut self) -> Result<(), ParseError> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after expression.")
    }

    fn variable(&mut self, can_assign: bool) -> Result<(), ParseError> {
        self.named_variable(self.previous().lexeme.to_string(), can_assign)
    }

    fn named_variable(&mut self, lexeme: String, can_assign: bool) -> Result<(), ParseError> {
        let arg = self.identifier_constant(lexeme)?;

        if can_assign && self.matches(TokenType::Equal) {
            self.expression()?;
            self.emit_bytes(OpCode::SetGlobal as u8, arg);
        } else {
            self.emit_bytes(OpCode::GetGlobal as u8, arg);
        }
        Ok(())
    }

    fn literal(&mut self) {
        match self.previous().kind {
            TokenType::True => {
                self.emit_byte(OpCode::True as u8);
                self.last_operand_type = ValueType::Bool;
            }
            TokenType::False => {
                self.emit_byte(OpCode::False as u8);
                self.last_operand_type = ValueType::Bool;
            }
            TokenType::Null => {
                self.emit_byte(OpCode::Null as u8);
                self.last_operand_type = ValueType::Null;
            }
            _ => unreachable!(),
        }
    }

    fn execute_fn_type(&mut self, fn_type: FnType, can_assign: bool) -> Result<(), ParseError> {
        match fn_type {
            FnType::Grouping => self.grouping(),
            FnType::Unary => self.unary(),
            FnType::Binary => self.binary(),
            FnType::Number => self.number(),
            FnType::String => self.string(),
            FnType::Variable => self.variable(can_assign),
            FnType::Literal => {
                self.literal();
                Ok(())
            }
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
        self.chunk.write_byte_to_chunk(byte, self.previous().line);
    }

    fn emit_bytes(&mut self, byte_0: u8, byte_1: u8) {
        self.emit_byte(byte_0);
        self.emit_byte(byte_1);
    }

    fn matches(&mut self, kind: TokenType) -> bool {
        if !self.check(kind) {
            false
        } else {
            self.advance();
            true
        }
    }

    fn check(&mut self, kind: TokenType) -> bool {
        self.peek().kind == kind
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
