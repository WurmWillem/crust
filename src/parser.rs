use colored::Colorize;

use crate::{
    compiler_types::*,
    error::{print_error, ParseError, EXPECTED_SEMICOLON_MSG},
    parse_types::{BinaryOp, Expr, ExprType, If, Stmt, StmtType},
    token::{Literal, Token, TokenType},
    value::ValueType,
};

pub struct Parser<'token> {
    tokens: Vec<Token<'token>>,
    current_token: usize,
}
impl<'a> Parser<'a> {
    pub fn compile(tokens: Vec<Token<'a>>) -> Vec<Stmt<'a>> {
        let mut parser = Parser {
            tokens,
            current_token: 0,
        };

        let mut had_error = false;
        let mut statements = Vec::new();
        while !parser.check(TokenType::Eof) {
            match parser.declaration() {
                Ok(result) => {
                    statements.push(result);
                }
                Err(err) => {
                    print_error(err.line, &err.msg);
                    had_error = true;
                }
            }
        }
        parser.current_token += 1;

        if had_error {
            panic!(
                "{}",
                "Compile error(s) detected, terminating program.".purple()
            );
        }

        if parser.current_token != parser.tokens.len() {
            println!("{}", "Not all tokens were parsed.".red());
        }
        statements
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
        let kind = ExprType::Binary { left, op, right };
        let expr = Expr::new(kind, line);
        Ok(expr)
    }

    fn number(&mut self) -> Result<Expr<'a>, ParseError> {
        let Literal::Num(value) = self.previous().literal else {
            unreachable!();
        };
        let kind = ExprType::Lit(Literal::Num(value));
        Ok(Expr::new(kind, self.previous().line))
    }

    fn unary(&mut self) -> Result<Expr<'a>, ParseError> {
        let prefix = self.previous().kind;
        let value = Box::new(self.parse_precedence(Precedence::Unary)?);

        let line = self.previous().line;
        let kind = ExprType::Unary { prefix, value };
        let expr = Expr::new(kind, line);
        Ok(expr)
    }

    fn literal(&mut self) -> Result<Expr<'a>, ParseError> {
        let literal = match self.previous().kind {
            TokenType::True => Literal::True,
            TokenType::False => Literal::False,
            TokenType::Null => Literal::Null,
            _ => unreachable!(),
        };
        let kind = ExprType::Lit(literal);
        Ok(Expr::new(kind, self.previous().line))
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParseError> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn declaration(&mut self) -> Result<Stmt<'a>, ParseError> {
        if let Some(var_type) = self.peek().kind.as_value_type() {
            self.advance();
            self.var_decl(var_type)
        } else {
            self.statement()
        }
    }

    fn var_decl(&mut self, ty: ValueType) -> Result<Stmt<'a>, ParseError> {
        self.consume(TokenType::Identifier, "Expected variable name.")?;
        let name = self.previous().lexeme;
        let line = self.previous().line;

        let value = if self.matches(TokenType::Equal) {
            let value = self.expression()?;
            self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
            value
        } else {
            self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
            Expr::new(ExprType::Lit(Literal::Null), line)
        };

        let kind = StmtType::Var { name, value, ty };
        let var = Stmt::new(kind, line);
        return Ok(var);
    }

    fn statement(&mut self) -> Result<Stmt<'a>, ParseError> {
        if self.matches(TokenType::Print) {
            self.print_statement()
        } else if self.matches(TokenType::LeftBrace) {
            self.block()
        } else if self.matches(TokenType::If) {
            self.if_stmt()
        } else {
            self.expr_stmt()
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

    fn if_stmt(&mut self) -> Result<Stmt<'a>, ParseError> {
        let line = self.previous().line;

        let first_condition = self.expression()?;
        let first_body = self.statement()?;
        let first_if = Box::new(If::new(first_condition, first_body));

        let mut final_else = None;
        if self.matches(TokenType::Else) {
            final_else = Some(Box::new(self.statement()?));
        }

        let ty = StmtType::If {
            first_if,
            final_else,
        };
        Ok(Stmt::new(ty, line))
    }

    fn print_statement(&mut self) -> Result<Stmt<'a>, ParseError> {
        let kind = StmtType::Println(self.expression()?);
        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;

        let stmt = Stmt::new(kind, self.previous().line);
        Ok(stmt)
    }

    fn expr_stmt(&mut self) -> Result<Stmt<'a>, ParseError> {
        let kind = StmtType::Expr(self.expression()?);
        let stmt = Stmt::new(kind, self.previous().line);
        Ok(stmt)
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

    fn block(&mut self) -> Result<Stmt<'a>, ParseError> {
        let mut stmts = vec![];
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' at end of block.")?;

        let ty = StmtType::Block(stmts);
        let block = Stmt::new(ty, self.previous().line);
        Ok(block)
    }
    fn var(&mut self, can_assign: bool) -> Result<Expr<'a>, ParseError> {
        let name = self.previous();
        let var = if can_assign && self.matches(TokenType::Equal) {
            let value = Box::new(self.expression()?);
            let ty = ExprType::Assign {
                name: name.lexeme,
                value,
            };
            self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
            Expr::new(ty, name.line)
        } else {
            let ty = ExprType::Var(name.lexeme);
            Expr::new(ty, name.line)
        };
        Ok(var)
    }

    fn resolve_local(&mut self, _name: &str, _can_assign: bool) -> Result<bool, ParseError> {
        todo!()
    }

    fn string(&mut self) -> Result<Expr<'a>, ParseError> {
        let Literal::Str(value) = self.previous().literal else {
            unreachable!();
        };
        let kind = ExprType::Lit(Literal::Str(value));
        Ok(Expr::new(kind, self.previous().line))
    }

    fn grouping(&mut self) -> Result<Expr<'a>, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
        Ok(expr)
    }

    fn execute_prefix(
        &mut self,
        fn_type: FnType,
        can_assign: bool,
    ) -> Result<Expr<'a>, ParseError> {
        // dbg!(fn_type);
        match fn_type {
            FnType::Grouping => self.grouping(),
            FnType::Unary => self.unary(),
            FnType::Number => self.number(),
            FnType::String => self.string(),
            FnType::Literal => self.literal(),
            FnType::Var => self.var(can_assign),
            _ => unreachable!(),
        }
    }

    fn execute_infix(
        &mut self,
        left: Expr<'a>,
        fn_type: FnType,
        _can_assign: bool,
    ) -> Result<Expr<'a>, ParseError> {
        match fn_type {
            FnType::Binary => self.binary(left),
            _ => unreachable!(),
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
