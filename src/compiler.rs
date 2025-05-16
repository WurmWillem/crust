use colored::Colorize;

use crate::{
    compiler_types::*,
    error::{print_error, ParseError, EXPECTED_SEMICOLON_MSG},
    object::{Heap, ObjFunc, Object},
    opcode::OpCode,
    token::{Literal, Token, TokenType},
    value::{StackValue, ValueType},
    vm::MAX_FUNC_AMT,
};

pub struct Parser<'token> {
    tokens: Vec<Token<'token>>,
    current_token: usize,
    last_operand_type: ValueType,
    heap: Heap,
    comps: CompilerStack<'token>,
    funcs: DeclaredFuncStack<'token>,
}
impl<'token> Parser<'token> {
    pub fn compile(
        tokens: Vec<Token<'token>>,
    ) -> Option<(ObjFunc, Heap, [StackValue; MAX_FUNC_AMT])> {
        let mut parser = Parser {
            tokens,
            current_token: 0,
            heap: Heap::new(),
            last_operand_type: ValueType::None,
            comps: CompilerStack::new(),
            funcs: DeclaredFuncStack::new(),
        };

        let mut had_error = false;
        while !parser.matches(TokenType::Eof) {
            if let Err(err) = parser.declaration() {
                print_error(err.line, &err.msg);

               had_error = true;
                parser.synchronize();
            }
        }
        if had_error {
            println!("{}", "Parse error(s) detected, terminate program.".purple());
            return None;
        }

        if parser.current_token != parser.tokens.len() - 1 {
            println!("{}", "Not all tokens were parsed.".red());
        }

        let func = parser.end_compiler();
        let funcs = parser.funcs.to_stack_value_arr();

        Some((func, parser.heap, funcs))
    }

    fn end_compiler(&mut self) -> ObjFunc {
        self.emit_return();
        self.comps.pop().get_func()
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Null as u8);
        self.emit_byte(OpCode::Return as u8);
    }

    fn declaration(&mut self) -> Result<(), ParseError> {
        if self.matches(TokenType::Var) {
            self.var_declaration()
        } else if self.matches(TokenType::Fun) {
            self.func_declaration()
        } else {
            self.statement()
        }
    }

    fn func_declaration(&mut self) -> Result<(), ParseError> {
        self.consume(TokenType::Identifier, "Expected function name.")?;
        let name = self.previous();

        self.funcs.edit_name(name.lexeme);
        self.function(name.lexeme.to_string())?;

        Ok(())
    }
    fn function(&mut self, name: String) -> Result<(), ParseError> {
        self.comps.push(name.clone());
        self.begin_scope();

        self.consume(TokenType::LeftParen, "Expected '(' after function name.")?;

        // parse parameters
        if !self.check(TokenType::RightParen) {
            let var_type = match self.advance().kind.as_value_type() {
                Some(var_type) => var_type,
                _ => {
                    return Err(ParseError::new(
                        self.previous().line,
                        "Expected type for parameter.",
                    ));
                }
            };

            self.consume(TokenType::Identifier, "Expected variable name.")?;
            let name = self.previous();

            self.comps.add_local(name, var_type)?;
            self.comps.increment_arity();

            while self.matches(TokenType::Comma) {
                let var_type = match self.advance().kind.as_value_type() {
                    Some(var_type) => var_type,
                    _ => {
                        return Err(ParseError::new(
                            self.previous().line,
                            "Expected type for parameter.",
                        ));
                    }
                };

                self.consume(TokenType::Identifier, "Expected variable name.")?;
                let name = self.previous();

                self.comps.add_local(name, var_type)?;
                self.comps.increment_arity();
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters.")?;
        self.consume(TokenType::LeftBrace, "Expected '{' before function body.")?;

        self.block()?;

        self.emit_return();

        let func = self.end_compiler();
        let (func_object, _) = self.heap.alloc(func, Object::Func);

        let value = StackValue::Obj(func_object);
        self.funcs.edit_value_and_increment_top(value);

        Ok(())
    }

    fn var_declaration(&mut self) -> Result<(), ParseError> {
        self.consume(TokenType::Identifier, "Expected variable name.")?;
        let name = self.previous();

        if self.matches(TokenType::Equal) {
            self.expression()?;
            self.comps.add_local(name, self.last_operand_type)?;
        } else {
            self.emit_byte(OpCode::Null as u8);
            self.comps.add_local(name, ValueType::Null)?;
        }

        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
        Ok(())
    }

    fn statement(&mut self) -> Result<(), ParseError> {
        // dbg!(self.peek());
        if self.matches(TokenType::Print) {
            self.print_statement()
        } else if self.matches(TokenType::If) {
            self.if_statement()
        } else if self.matches(TokenType::While) {
            self.while_statement()
        } else if self.matches(TokenType::For) {
            self.for_statement()
        } else if self.matches(TokenType::Return) {
            self.return_statement()
        } else if self.matches(TokenType::LeftBrace) {
            self.begin_scope();
            self.block()?;
            self.end_scope();
            Ok(())
        } else {
            self.expression_statement()
        }
    }

    fn emit_loop(&mut self, loop_start: usize) -> Result<(), ParseError> {
        self.emit_byte(OpCode::Loop as u8);

        let offset = self.comps.get_code_len() - loop_start + 2;
        if offset > u8::MAX as usize {
            let msg = "Loop body too large.";
            return Err(ParseError::new(0, msg));
        }

        self.emit_byte(((offset >> 8) & 0xFF) as u8);
        self.emit_byte((offset & 0xFF) as u8);
        Ok(())
    }

    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit_byte(instruction as u8);
        // placeholders
        self.emit_byte(0xFF);
        self.emit_byte(0xFF);
        self.comps.get_code_len() - 2
    }

    fn return_statement(&mut self) -> Result<(), ParseError> {
        if self.matches(TokenType::Semicolon) {
            self.emit_return();
        } else {
            self.expression()?;
            self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
            self.emit_byte(OpCode::Return as u8);
        }
        Ok(())
    }

    fn for_statement(&mut self) -> Result<(), ParseError> {
        self.begin_scope();

        self.consume(TokenType::LeftParen, "Expected '(' after 'for'.")?;

        if self.matches(TokenType::Semicolon) {
        } else if self.matches(TokenType::Var) {
            self.var_declaration()?;
        } else {
            self.expression_statement()?;
        }
        let mut loop_start = self.comps.get_code_len();

        let mut exit_jump = None;
        if !self.matches(TokenType::Semicolon) {
            self.expression()?;
            self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

            exit_jump = Some(self.emit_jump(OpCode::JumpIfFalse));
            self.emit_byte(OpCode::Pop as u8);
        }

        if !self.matches(TokenType::RightParen) {
            let body_jump = self.emit_jump(OpCode::Jump);
            let increment_start = self.comps.get_code_len();

            self.expression()?;
            self.emit_byte(OpCode::Pop as u8);
            self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;

            self.emit_loop(loop_start)?;
            loop_start = increment_start;
            self.comps.patch_jump(body_jump)?;
        }

        self.statement()?;
        self.emit_loop(loop_start)?;

        if let Some(exit_jump) = exit_jump {
            self.comps.patch_jump(exit_jump)?;
            self.emit_byte(OpCode::Pop as u8);
        }

        self.end_scope();
        Ok(())
    }

    fn while_statement(&mut self) -> Result<(), ParseError> {
        let loop_start = self.comps.get_code_len();
        self.expression()?;

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_byte(OpCode::Pop as u8);
        self.statement()?;
        self.emit_loop(loop_start)?;

        self.comps.patch_jump(exit_jump)?;
        self.emit_byte(OpCode::Pop as u8);
        Ok(())
    }

    fn if_statement(&mut self) -> Result<(), ParseError> {
        self.expression()?;

        let then_jump = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_byte(OpCode::Pop as u8);
        self.statement()?;

        let else_jump = self.emit_jump(OpCode::Jump);

        self.comps.patch_jump(then_jump)?;
        self.emit_byte(OpCode::Pop as u8);

        if self.matches(TokenType::Else) {
            self.statement()?;
        }
        self.comps.patch_jump(else_jump)
    }

    fn print_statement(&mut self) -> Result<(), ParseError> {
        // dbg!(self.chunk.constants.len());
        self.expression()?;
        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
        self.emit_byte(OpCode::Print as u8);
        // dbg!(self.chunk.constants.len());
        Ok(())
    }

    fn expression_statement(&mut self) -> Result<(), ParseError> {
        self.expression()?;
        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
        self.emit_byte(OpCode::Pop as u8);
        Ok(())
    }

    fn synchronize(&mut self) {
        self.advance();

        while self.peek().kind != TokenType::Eof {
            // if we just consumed a semicolon, we probably ended a statement
            if self.previous().kind == TokenType::Semicolon {
                dbg!("semi");
                return;
            }

            // check if next token looks like the start of a new statement
            match self.peek().kind {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    dbg!(self.peek().kind);
                    return
                },
                _ => (),
            }

            self.advance();
        }
        if self.comps.get_scope_depth() == 0 {
            while self.peek().kind != TokenType::Eof && self.previous().kind != TokenType::Semicolon
            {
                self.advance();
            }
        } else {
            let mut brace_count = 0;
            while self.peek().kind != TokenType::Eof {
                if self.previous().kind == TokenType::LeftBrace {
                    brace_count += 1;
                }
                if self.previous().kind == TokenType::RightBrace {
                    brace_count -= 1;
                }
                if brace_count < 0 {
                    break;
                }
                self.advance();
            }
        }
    }

    fn block(&mut self) -> Result<(), ParseError> {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            self.declaration()?;
        }
        self.consume(TokenType::RightBrace, "Expected '}' at end of block.")
    }

    fn begin_scope(&mut self) {
        self.comps.increment_scope_depth();
    }

    fn end_scope(&mut self) {
        self.comps.decrement_scope_depth();

        while self.comps.should_remove_local() {
            self.emit_byte(OpCode::Pop as u8);
            self.comps.decrement_local_count()
        }
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

    fn variable(&mut self, can_assign: bool) -> Result<(), ParseError> {
        let name = self.previous();

        if let Some((arg, kind)) = self.comps.resolve_local(&name.lexeme) {
            if can_assign && self.matches(TokenType::Equal) {
                self.expression()?;
                self.emit_bytes(OpCode::SetLocal as u8, arg);
            } else {
                self.emit_bytes(OpCode::GetLocal as u8, arg);
                self.last_operand_type = kind;
            }
            return Ok(());
        }

        if let Some(arg) = self.funcs.resolve_func(&name.lexeme) {
            self.emit_bytes(OpCode::GetFunc as u8, arg);
            return Ok(());
        }

        let msg = format!(
            "The variable/function with name '{}' does not exist.",
            name.lexeme
        );
        Err(ParseError::new(name.line, &msg))
    }

    fn string(&mut self) -> Result<(), ParseError> {
        let Literal::Str(value) = self.previous().literal else {
            unreachable!();
        };
        let (object, _) = self.heap.alloc(value.to_string(), Object::Str);
        let stack_value = StackValue::Obj(object);

        self.last_operand_type = ValueType::Str;
        self.emit_constant(stack_value)
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
                    let lhs_type = lhs_type.to_string();
                    let rhs_type = self.last_operand_type.to_string();
                    let msg = &format!(
                        "Operator '{}' expects two numbers, but got types '{}' and '{}'.",
                        $op_char, lhs_type, rhs_type
                    );
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
                    let lhs_type = lhs_type.to_string();
                    let rhs_type = self.last_operand_type.to_string();
                    let msg = format!(
                        "Operator '+' expects two numbers or two strings, but got types '{}' and '{}'.",
                        lhs_type, rhs_type
                    );
                    return Err(ParseError::new(self.peek().line, &msg));
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
            FnType::Call => self.call(),
        }
    }

    fn call(&mut self) -> Result<(), ParseError> {
        let arg_count = self.argument_list()?;
        dbg!(arg_count);
        self.emit_bytes(OpCode::Call as u8, arg_count + 1);
        Ok(())
    }
    fn argument_list(&mut self) -> Result<u8, ParseError> {
        let mut arg_count = 0;

        while !self.check(TokenType::RightParen) {
            self.expression()?;
            arg_count += 1;

            if !self.matches(TokenType::Comma) {
                break;
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after argument list")?;
        Ok(arg_count)
    }

    fn emit_constant(&mut self, value: StackValue) -> Result<(), ParseError> {
        let const_index = self.make_constant(value)?;
        self.emit_bytes(OpCode::Constant as u8, const_index);
        Ok(())
    }

    fn make_constant(&mut self, value: StackValue) -> Result<u8, ParseError> {
        let const_index = self.comps.add_constant(value);
        if const_index > u16::MAX.into() {
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
        let line = self.previous().line;
        self.comps.write_byte_to_chunk(byte, line);
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

    fn check(&self, kind: TokenType) -> bool {
        self.peek().kind == kind
    }

    fn advance(&mut self) -> Token<'token> {
        if self.peek().kind != TokenType::Eof {
            self.current_token += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token<'token> {
        self.tokens[self.current_token]
    }

    fn previous(&self) -> Token<'token> {
        self.tokens[self.current_token - 1]
    }
}
