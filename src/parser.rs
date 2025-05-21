use colored::Colorize;

use crate::{
    compiler_types::*,
    error::{print_error, ParseError},
    opcode::OpCode,
    token::{Literal, Token, TokenType},
    value::ValueType,
};

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,

    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    And,
    Or,
}
impl BinaryOp {
    fn get_precedency(self) -> Precedence {
        match self {
            BinaryOp::Add | BinaryOp::Sub => Precedence::Term,
            BinaryOp::Mul | BinaryOp::Div => Precedence::Factor,
            BinaryOp::Equal | BinaryOp::NotEqual => Precedence::Equality,
            BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                Precedence::Comparison
            }
            BinaryOp::And => Precedence::And,
            BinaryOp::Or => Precedence::Or,
        }
    }
    fn from_token_type(ty: TokenType) -> Self {
        match ty {
            TokenType::Plus => BinaryOp::Add,
            TokenType::Minus => BinaryOp::Sub,
            TokenType::Star => BinaryOp::Mul,
            TokenType::Slash => BinaryOp::Div,
            TokenType::EqualEqual => BinaryOp::Equal,
            TokenType::BangEqual => BinaryOp::NotEqual,
            TokenType::Less => BinaryOp::Less,
            TokenType::LessEqual => BinaryOp::LessEqual,
            TokenType::Greater => BinaryOp::Greater,
            TokenType::GreaterEqual => BinaryOp::GreaterEqual,
            _ => unreachable!(),
        }
    }
    pub fn to_op_code(self) -> OpCode {
        match self {
            BinaryOp::Add => OpCode::Add,
            BinaryOp::Sub => OpCode::Sub,
            BinaryOp::Mul => OpCode::Mul,
            BinaryOp::Div => OpCode::Div,
            BinaryOp::Equal => OpCode::Equal,
            BinaryOp::NotEqual => OpCode::NotEqual,
            BinaryOp::Less => OpCode::Less,
            BinaryOp::LessEqual => OpCode::LessEqual,
            BinaryOp::Greater => OpCode::Greater,
            BinaryOp::GreaterEqual => OpCode::GreaterEqual,
            _ => todo!(), // BinaryOp::And => OpCode::And,
                          // BinaryOp::GreaterEqual => OpCode::GreaterEqual,
        }
    }
}

#[derive(Debug)]
pub enum Expr<'a> {
    Lit(Literal<'a>, u32),
    Variable(String),
    Unary {
        prefix: TokenType,
        value: Box<Expr<'a>>,
        line: u32,
    },
    Binary {
        left: Box<Expr<'a>>,
        op: BinaryOp,
        right: Box<Expr<'a>>,
        line: u32,
    },
}

pub struct Parser<'token> {
    tokens: Vec<Token<'token>>,
    current_token: usize,
}
impl<'a> Parser<'a> {
    pub fn compile(tokens: Vec<Token<'a>>) -> Expr {
        let mut parser = Parser {
            tokens,
            current_token: 0,
        };

        let mut had_error = false;
        let mut result = Expr::Lit(Literal::Null, 0);
        while !parser.check(TokenType::Eof) {
            let parsed = parser.expression();
            result = match parsed {
                Ok(result) => result,
                Err(err) => {
                    print_error(err.line, &err.msg);

                    had_error = true;
                    Expr::Lit(Literal::Null, 0)
                }
            }
        }
        // parser.current_token += 1;
        // dbg!(&parser.decl_types);

        if had_error {
            panic!(
                "{}",
                "Compile error(s) detected, terminating program.".purple()
            );
        }

        if parser.current_token != parser.tokens.len() {
            println!("{}", "Not all tokens were parsed.".red());
        }
        result
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Expr<'a>, ParseError> {
        self.advance();
        let (can_assign, mut expr) = self.parse_prefix(precedence)?;

        while self.peek().kind != TokenType::Eof
            && precedence <= self.get_rule(self.peek().kind).precedence
        {
            self.advance();
            let infix = self.get_rule(self.previous().kind).infix;
            expr = self.execute_infix(expr, infix, can_assign)?;
        }
        Ok(expr)
    }

    fn parse_prefix(&mut self, precedence: Precedence) -> Result<(bool, Expr<'a>), ParseError> {
        let kind = self.previous().kind;
        dbg!(kind);

        let prefix = self.get_rule(kind).prefix;
        if prefix == FnType::Empty {
            let msg = "Expected expression.";
            let err = ParseError::new(self.previous().line, msg);
            return Err(err);
        }

        let can_assign = precedence <= Precedence::Assignment;
        let expr = self.execute_prefix(prefix, can_assign)?;
        Ok((can_assign, expr))
    }

    fn binary(&mut self, left: Expr<'a>) -> Result<Expr<'a>, ParseError> {
        let left = Box::new(left);
        let op = BinaryOp::from_token_type(self.previous().kind);

        let precedence = op.get_precedency();
        let right = Box::new(self.parse_precedence(precedence)?);

        let line = self.previous().line;
        Ok(Expr::Binary {
            left,
            op,
            right,
            line,
        })
    }

    fn number(&mut self) -> Result<Expr<'a>, ParseError> {
        let Literal::Num(value) = self.previous().literal else {
            unreachable!();
        };
        Ok(Expr::Lit(Literal::Num(value), self.previous().line))
    }

    fn unary(&mut self) -> Result<Expr<'a>, ParseError> {
        let prefix = self.previous().kind;
        let right = Box::new(self.parse_precedence(Precedence::Unary)?);
        let line = self.previous().line;

        Ok(Expr::Unary {
            prefix,
            value: right,
            line,
        })
    }

    fn literal(&mut self) -> Result<Expr<'a>, ParseError> {
        let literal = match self.previous().kind {
            TokenType::True => Literal::True,
            TokenType::False => Literal::False,
            TokenType::Null => Literal::Null,
            _ => unreachable!(),
        };
        let line = self.previous().line;
        Ok(Expr::Lit(literal, line))
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParseError> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn declaration(&mut self) -> Result<(), ParseError> {
        if let Some(var_type) = self.peek().kind.as_value_type() {
            self.advance();
            self.var_decl(var_type)
        } else {
            self.statement()
        }
    }

    fn var_decl(&mut self, _var_type: ValueType) -> Result<(), ParseError> {
        todo!()
    }

    fn statement(&mut self) -> Result<(), ParseError> {
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
            todo!()
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn for_statement(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn while_statement(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn if_statement(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn print_statement(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn expression_statement(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn synchronize(&mut self) {
        self.advance();
        // dbg!(self.peek().kind);

        while self.peek().kind != TokenType::Eof {
            // if we just consumed a semicolon, we probably ended a statement
            if self.previous().kind == TokenType::Semicolon
                && self.peek().kind != TokenType::RightBrace
            {
                return;
            }

            // check if next token looks like the start of a new statement
            match self.peek().kind {
                TokenType::Struct
                | TokenType::Fun
                // | TokenType::F64 used to be 'let', but these are also used in the middle of statments
                // | TokenType::Bool
                // | TokenType::Str
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    // dbg!(self.peek().kind);
                    return;
                }
                _ => (),
            }

            self.advance();
        }
    }

    fn block(&mut self) -> Result<(), ParseError> {
        todo!()
    }
    fn var_or_func(&mut self, _can_assign: bool) -> Result<(), ParseError> {
        todo!()
    }

    fn resolve_local(&mut self, _name: &str, _can_assign: bool) -> Result<bool, ParseError> {
        todo!()
    }

    fn string(&mut self) -> Result<Expr<'a>, ParseError> {
        todo!()
    }

    fn grouping(&mut self) -> Result<Expr<'a>, ParseError> {
        todo!()
    }

    fn execute_prefix(
        &mut self,
        fn_type: FnType,
        _can_assign: bool,
    ) -> Result<Expr<'a>, ParseError> {
        // dbg!(fn_type);
        match fn_type {
            // FnType::Grouping => self.grouping(),
            FnType::Unary => self.unary(),
            // FnType::Binary => self.binary(),
            FnType::Number => self.number(),
            FnType::String => self.string(),
            // FnType::Variable => self.var_or_func(can_assign),
            FnType::Literal => {
                self.literal()
                // Ok(())
            }
            // FnType::Empty => Ok(()),
            // FnType::Call => unreachable!(),
            _ => todo!(),
        }
    }

    fn execute_infix(
        &mut self,
        left: Expr<'a>,
        fn_type: FnType,
        _can_assign: bool,
    ) -> Result<Expr<'a>, ParseError> {
        match fn_type {
            // FnType::Grouping => self.grouping(),
            FnType::Unary => self.unary(),
            FnType::Binary => self.binary(left),
            FnType::Number => self.number(),
            FnType::String => self.string(),
            // FnType::Variable => self.var_or_func(can_assign),
            FnType::Literal => {
                self.literal()
                // Ok(())
            }
            // FnType::Empty => Ok(()),
            // FnType::Call => unreachable!(),
            _ => todo!(),
        }
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

    fn advance(&mut self) -> Token<'a> {
        if self.peek().kind != TokenType::Eof {
            self.current_token += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token<'a> {
        self.tokens[self.current_token]
    }

    fn previous(&self) -> Token<'a> {
        self.tokens[self.current_token - 1]
    }
}
