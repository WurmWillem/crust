use colored::Colorize;

use crate::{
    compiler_types::*,
    error::{print_error, ParseError, EXPECTED_SEMICOLON_MSG},
    parse_types::{BinaryOp, Expr, ExprType, Stmt, StmtType},
    token::{Literal, Token, TokenType},
    value::ValueType,
};

pub struct Parser<'token> {
    tokens: Vec<Token<'token>>,
    current_token: usize,
}
impl<'a> Parser<'a> {
    pub fn compile(tokens: Vec<Token<'a>>) -> Option<Vec<Stmt<'a>>> {
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
                    parser.synchronize();
                }
            }
        }
        parser.current_token += 1;

        if had_error {
            return None;
        }

        if parser.current_token != parser.tokens.len() {
            println!("{}", "Not all tokens were parsed.".red());
        }
        Some(statements)
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
        } else if self.matches(TokenType::Fn) {
            self.func_decl()
        } else {
            self.statement()
        }
    }

    fn func_decl(&mut self) -> Result<Stmt<'a>, ParseError> {
        self.consume(TokenType::Identifier, "Expected variable name.")?;
        let name = self.previous().lexeme;
        let line = self.previous().line;

        self.consume(TokenType::LeftParen, "Expected '(' after function name.")?;

        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            parameters.push(self.parse_parameter()?);
            while self.matches(TokenType::Comma) {
                parameters.push(self.parse_parameter()?);
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after function name.")?;

        let mut return_ty = ValueType::Null;
        if self.matches(TokenType::Colon) {
            return_ty = match self.advance().kind.as_value_type() {
                Some(return_ty) => return_ty,
                _ => {
                    return Err(ParseError::new(
                        self.previous().line,
                        "Expected return type after finding ':'.",
                    ));
                }
            };
        }

        let body = Box::new(self.statement()?);
        let fn_ty = StmtType::Func {
            name,
            parameters,
            body,
            return_ty,
        };
        let func = Stmt::new(fn_ty, line);
        Ok(func)
    }
    fn parse_parameter(&mut self) -> Result<(ValueType, &'a str), ParseError> {
        let var_type = match self.advance().kind.as_value_type() {
            Some(var_type) => var_type,
            _ => {
                return Err(ParseError::new(
                    self.previous().line,
                    "Expected type for parameter.",
                ));
            }
        };

        self.consume(TokenType::Identifier, "Expected parameter name.")?;
        let name = self.previous().lexeme;

        Ok((var_type, name))
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
        } else if self.matches(TokenType::While) {
            self.while_stmt()
        } else if self.matches(TokenType::For) {
            self.for_stmt()
        } else if self.matches(TokenType::Return) {
            self.return_statement()
        } else {
            self.expr_stmt()
        }
    }

    fn return_statement(&mut self) -> Result<Stmt<'a>, ParseError> {
        let value_ty = ExprType::Lit(Literal::Null);
        let mut value = Expr::new(value_ty, self.previous().line);

        if !self.check(TokenType::Semicolon) {
            value = self.expression()?;
        }

        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;

        let stmt_ty = StmtType::Return(value);
        let stmt = Stmt::new(stmt_ty, self.previous().line);
        Ok(stmt)
    }

    fn for_stmt(&mut self) -> Result<Stmt<'a>, ParseError> {
        self.consume(TokenType::Identifier, "Expected variable name after 'for'.")?;
        let var = self.previous();
        let line = var.line;
        let name = var.lexeme;

        self.consume(TokenType::In, "Expected 'in' after 'for identifier'.")?;

        // declare var
        let value = self.expression()?;
        let ty = ValueType::Num;
        let kind = StmtType::Var { name, value, ty };
        let var = Box::new(Stmt::new(kind, line));

        // condition
        self.consume(TokenType::To, "Expected 'to' after 'for identifier'.")?;
        let end = Box::new(self.expression()?);
        let get_var_ty = ExprType::Var(name);
        let get_var = Box::new(Expr::new(get_var_ty, line));
        let condition_ty = ExprType::Binary {
            left: get_var,
            op: BinaryOp::Less,
            right: end,
        };
        let condition = Expr::new(condition_ty, line);

        // produce for stmt
        let body = Box::new(self.statement()?);
        let for_ty = StmtType::For {
            condition,
            body,
            var,
        };
        let stmt = Stmt::new(for_ty, line);

        Ok(stmt)
    }

    fn while_stmt(&mut self) -> Result<Stmt<'a>, ParseError> {
        let condition = self.expression()?;
        let body = Box::new(self.statement()?);

        let ty = StmtType::While { condition, body };
        let stmt = Stmt::new(ty, self.previous().line);
        Ok(stmt)
    }

    fn if_stmt(&mut self) -> Result<Stmt<'a>, ParseError> {
        let line = self.previous().line;

        let condition = self.expression()?;
        let body = Box::new(self.statement()?);

        let mut final_else = None;
        if self.matches(TokenType::Else) {
            final_else = Some(Box::new(self.statement()?));
        }

        let ty = StmtType::If {
            condition,
            body,
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
        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
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
                | TokenType::Fn
                | TokenType::F64
                | TokenType::Bool
                | TokenType::Str
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

    fn call(&mut self, name: Expr<'a>) -> Result<Expr<'a>, ParseError> {
        let mut args = Vec::new();
        while !self.check(TokenType::RightParen) {
            args.push(self.expression()?);

            if !self.matches(TokenType::Comma) {
                break;
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after function call.")?;

        if let ExprType::Var(name) = name.expr {
            let ty = ExprType::Call { name, args };
            let expr = Expr::new(ty, self.previous().line);
            Ok(expr)
        } else {
            unreachable!()
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
            FnType::Call => self.call(left),
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
