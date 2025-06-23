use crate::{
    error::{print_error, ParseErr},
    expression::{Expr, ExprType},
    parse_types::{BinaryOp, FnType, ParseRule, Precedence, PARSE_RULES},
    statement::{Stmt, StmtType},
    token::{Literal, Token, TokenType},
    value::ValueType,
};

use colored::Colorize;

const EXPECTED_SEMICOLON_MSG: &str = "Expected ';' at end of statement.";

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

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Expr<'a>, ParseErr> {
        self.advance();
        let (can_assign, mut expr) = self.parse_prefix(precedence)?;

        while self.peek().ty != TokenType::Eof
            && precedence <= self.get_rule(self.peek().ty).precedence
        {
            self.advance();
            let infix = self.get_rule(self.previous().ty).infix;
            expr = self.execute_infix(expr, infix, can_assign)?;
        }
        Ok(expr)
    }

    fn parse_prefix(&mut self, precedence: Precedence) -> Result<(bool, Expr<'a>), ParseErr> {
        let kind = self.previous().ty;

        // dbg!(kind);
        let prefix = self.get_rule(kind).prefix;
        if prefix == FnType::Empty {
            let msg = "Expected expression.";
            let err = ParseErr::new(self.previous().line, msg);
            return Err(err);
        }

        let can_assign = precedence <= Precedence::Assignment;
        let expr = self.execute_prefix(prefix, can_assign)?;
        Ok((can_assign, expr))
    }

    fn binary(&mut self, left: Expr<'a>) -> Result<Expr<'a>, ParseErr> {
        let left = Box::new(left);
        let op = BinaryOp::from_token_type(self.previous().ty);

        let precedence = op.get_precedency();
        let right = Box::new(self.parse_precedence(precedence)?);

        let line = self.previous().line;
        let kind = ExprType::Binary { left, op, right };
        let expr = Expr::new(kind, line);
        Ok(expr)
    }

    fn array(&mut self) -> Result<Expr<'a>, ParseErr> {
        let mut values = Vec::new();
        while !self.check(TokenType::RightBracket) {
            values.push(self.expression()?);

            if !self.matches(TokenType::Comma) {
                break;
            }
        }
        self.consume(TokenType::RightBracket, "Expected ']' at end of array.")?;

        let ty = ExprType::Array(values);
        Ok(Expr::new(ty, self.previous().line))
    }

    fn number(&mut self) -> Result<Expr<'a>, ParseErr> {
        let kind = match self.previous().literal {
            Literal::F64(n) => ExprType::Lit(Literal::F64(n)),
            Literal::I64(n) => ExprType::Lit(Literal::I64(n)),
            Literal::U64(n) => ExprType::Lit(Literal::U64(n)),
            _ => unreachable!(),
        };
        Ok(Expr::new(kind, self.previous().line))
    }

    fn unary(&mut self) -> Result<Expr<'a>, ParseErr> {
        let prefix = self.previous().ty;
        let value = Box::new(self.parse_precedence(Precedence::Unary)?);

        let line = self.previous().line;
        let kind = ExprType::Unary { prefix, value };
        let expr = Expr::new(kind, line);
        Ok(expr)
    }

    fn literal(&mut self) -> Result<Expr<'a>, ParseErr> {
        let literal = match self.previous().ty {
            TokenType::True => Literal::True,
            TokenType::False => Literal::False,
            TokenType::Null => Literal::Null,
            _ => unreachable!(),
        };
        let kind = ExprType::Lit(literal);
        Ok(Expr::new(kind, self.previous().line))
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParseErr> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn declaration(&mut self) -> Result<Stmt<'a>, ParseErr> {
        if let Some(var_type) = self.peek().as_value_type() {
            self.advance();
            if self.peek().ty != TokenType::Identifier && self.peek().ty != TokenType::LeftBracket {
                self.regress();
                return self.statement();
            }

            self.advance();
            if self.peek().ty == TokenType::Num {
                self.regress();
                self.regress();
                return self.statement();
            }
            self.regress();
            self.var_decl(var_type)
        } else if self.matches(TokenType::Fn) {
            self.func_decl()
        } else if self.matches(TokenType::Struct) {
            self.struct_decl()
        } else {
            self.statement()
        }
    }

    fn struct_decl(&mut self) -> Result<Stmt<'a>, ParseErr> {
        self.consume(
            TokenType::Identifier,
            "Expected struct name after 'struct' keyword.",
        )?;
        let name = self.previous().lexeme;
        let line = self.previous().line;

        self.consume(TokenType::LeftBrace, "Expected '{' after struct name.")?;

        let mut fields = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Fn) {
            let mut field_ty = match self.advance().as_value_type() {
                Some(ty) => ty,
                None => {
                    let msg = "Expected type for field declaration in struct body.";
                    return Err(ParseErr::new(line, msg));
                }
            };
            while self.matches(TokenType::LeftBracket) {
                self.consume(TokenType::RightBracket, "Expected ']' after left bracket.")?;
                field_ty = ValueType::Arr(Box::new(field_ty));
            }

            self.consume(TokenType::Identifier, "Expected variable name after type.")?;
            let field_name = self.previous().lexeme;

            fields.push((field_ty, field_name));

            self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
        }
        let mut methods = vec![];
        while self.matches(TokenType::Fn) {
            methods.push(self.func_decl()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after struct body.")?;

        let ty = StmtType::Struct {
            name,
            fields,
            methods,
        };
        Ok(Stmt::new(ty, line))
    }

    fn func_decl(&mut self) -> Result<Stmt<'a>, ParseErr> {
        self.consume(
            TokenType::Identifier,
            "Expected function name after 'fn' keyword.",
        )?;
        let name = self.previous().lexeme;
        let line = self.previous().line;

        self.consume(TokenType::LeftParen, "Expected '(' after function name.")?;

        let mut parameters = Vec::new();
        let mut use_self = false;
        if !self.check(TokenType::RightParen) {
            if self.matches(TokenType::This) {
                use_self = true;
            }
            
            if !self.check(TokenType::RightParen) {
                parameters.push(self.parse_parameter()?);
                while self.matches(TokenType::Comma) {
                    parameters.push(self.parse_parameter()?);
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after function name.")?;

        let mut return_ty = ValueType::Null;
        if self.matches(TokenType::Colon) {
            return_ty = match self.advance().as_value_type() {
                Some(return_ty) => return_ty,
                _ => {
                    return Err(ParseErr::new(
                        self.previous().line,
                        "Expected return type after finding ':'.",
                    ));
                }
            };
        }

        self.consume(
            TokenType::LeftBrace,
            "Expected '{' at begin of function body.",
        )?;

        let mut body = vec![];
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            body.push(self.declaration()?);
        }

        if self.peek().ty != TokenType::Eof {
            self.consume(
                TokenType::RightBrace,
                "Expected '}' at end of function body.",
            )?;
        }

        let fn_ty = StmtType::Func {
            name,
            parameters,
            body,
            return_ty,
            use_self,
        };
        let func = Stmt::new(fn_ty, line);
        Ok(func)
    }
    fn parse_parameter(&mut self) -> Result<(ValueType, &'a str), ParseErr> {
        let var_ty = match self.advance().as_value_type() {
            Some(mut var_type) => {
                while self.matches(TokenType::LeftBracket) {
                    self.consume(TokenType::RightBracket, "Expected ']' after left bracket.")?;
                    var_type = ValueType::Arr(Box::new(var_type));
                }
                var_type
            }
            _ => {
                return Err(ParseErr::new(
                    self.previous().line,
                    "Expected type for parameter.",
                ));
            }
        };

        self.consume(TokenType::Identifier, "Expected parameter name.")?;
        let name = self.previous().lexeme;

        Ok((var_ty, name))
    }

    fn var_decl(&mut self, mut ty: ValueType) -> Result<Stmt<'a>, ParseErr> {
        while self.matches(TokenType::LeftBracket) {
            self.consume(TokenType::RightBracket, "Expected ']' after left bracket.")?;
            ty = ValueType::Arr(Box::new(ty));
        }

        self.consume(TokenType::Identifier, "Expected variable name after type.")?;
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
        Ok(var)
    }

    fn statement(&mut self) -> Result<Stmt<'a>, ParseErr> {
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
        } else if self.matches(TokenType::Break) {
            self.break_stmt()
        } else if self.matches(TokenType::Continue) {
            self.continue_stmt()
        } else if self.matches(TokenType::Return) {
            self.return_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn continue_stmt(&mut self) -> Result<Stmt<'a>, ParseErr> {
        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
        let stmt = Stmt::new(StmtType::Continue, self.previous().line);
        Ok(stmt)
    }

    fn break_stmt(&mut self) -> Result<Stmt<'a>, ParseErr> {
        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
        let stmt = Stmt::new(StmtType::Break, self.previous().line);
        Ok(stmt)
    }

    fn return_stmt(&mut self) -> Result<Stmt<'a>, ParseErr> {
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

    fn for_stmt(&mut self) -> Result<Stmt<'a>, ParseErr> {
        self.consume(TokenType::Identifier, "Expected variable name after 'for'.")?;
        let var = self.previous();
        let line = var.line;
        let name = var.lexeme;

        self.consume(TokenType::In, "Expected 'in' after 'for identifier'.")?;

        // declare var
        let value = self.expression()?;
        let ty = ValueType::I64;
        let kind = StmtType::Var { name, value, ty };
        let var = Box::new(Stmt::new(kind, line));

        // condition
        self.consume(TokenType::To, "Expected 'to' after 'for identifier'.")?;
        let end = Box::new(self.expression()?);
        let cast = ExprType::Cast {
            value: end,
            target: ValueType::I64,
        };
        let cast = Expr::new(cast, line);

        let get_var_ty = ExprType::Var(name);
        let get_var = Box::new(Expr::new(get_var_ty, line));
        let condition_ty = ExprType::Binary {
            left: get_var,
            op: BinaryOp::Less,
            right: Box::new(cast),
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

    fn while_stmt(&mut self) -> Result<Stmt<'a>, ParseErr> {
        let condition = self.expression()?;
        let body = Box::new(self.statement()?);

        let ty = StmtType::While { condition, body };
        let stmt = Stmt::new(ty, self.previous().line);
        Ok(stmt)
    }

    fn if_stmt(&mut self) -> Result<Stmt<'a>, ParseErr> {
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

    fn print_statement(&mut self) -> Result<Stmt<'a>, ParseErr> {
        let kind = StmtType::Println(self.expression()?);
        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;

        let stmt = Stmt::new(kind, self.previous().line);
        Ok(stmt)
    }

    fn expr_stmt(&mut self) -> Result<Stmt<'a>, ParseErr> {
        let kind = StmtType::Expr(self.expression()?);
        let stmt = Stmt::new(kind, self.previous().line);
        self.consume(TokenType::Semicolon, EXPECTED_SEMICOLON_MSG)?;
        Ok(stmt)
    }

    fn synchronize(&mut self) {
        // self.advance();

        while self.peek().ty != TokenType::Eof {
            // if we just consumed a semicolon, we probably ended a statement
            if self.previous().ty == TokenType::Semicolon && self.peek().ty != TokenType::RightBrace
            {
                return;
            }

            // check if next token looks like the start of a new statement
            match self.peek().ty {
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

    fn block(&mut self) -> Result<Stmt<'a>, ParseErr> {
        let mut stmts = vec![];
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' at end of block.")?;

        let ty = StmtType::Block(stmts);
        let block = Stmt::new(ty, self.previous().line);
        Ok(block)
    }

    fn this(&mut self) -> Result<Expr<'a>, ParseErr> {
        Ok(Expr::new(ExprType::This, self.previous().line))
    }

    fn var(&mut self, can_assign: bool) -> Result<Expr<'a>, ParseErr> {
        let name = self.previous().lexeme;
        let line = self.previous().line;

        let ty = if can_assign && self.matches(TokenType::Equal) {
            let value = Box::new(self.expression()?);
            ExprType::Assign {
                name,
                new_value: value,
            }
        } else if can_assign && self.matches(TokenType::PlusEqual) {
            self.get_assign_shorthand(name, line, BinaryOp::Add)?
        } else if can_assign && self.matches(TokenType::MinEqual) {
            self.get_assign_shorthand(name, line, BinaryOp::Sub)?
        } else if can_assign && self.matches(TokenType::MulEqual) {
            self.get_assign_shorthand(name, line, BinaryOp::Mul)?
        } else if can_assign && self.matches(TokenType::DivEqual) {
            self.get_assign_shorthand(name, line, BinaryOp::Div)?
        } else {
            ExprType::Var(name)
        };
        let var = Expr::new(ty, line);
        Ok(var)
    }
    fn get_assign_shorthand(
        &mut self,
        name: &'a str,
        line: u32,
        op: BinaryOp,
    ) -> Result<ExprType<'a>, ParseErr> {
        let var_ty = ExprType::Var(name);
        let var = Box::new(Expr::new(var_ty, line));

        let operand = Box::new(self.expression()?);
        let ty = ExprType::Binary {
            left: var,
            op,
            right: operand,
        };

        let new_value = Box::new(Expr::new(ty, line));
        Ok(ExprType::Assign { name, new_value })
    }

    fn get_assign_shorthand_field(
        &mut self,
        field_name: &'a str,
        line: u32,
        op: BinaryOp,
        inst: Expr<'a>,
    ) -> Result<ExprType<'a>, ParseErr> {
        let ty = ExprType::Dot {
            inst: Box::new(inst.clone()),
            property: field_name,
        };
        let left = Box::new(Expr::new(ty, line));

        let operand = Box::new(self.expression()?);
        let ty = ExprType::Binary {
            left,
            op,
            right: operand,
        };

        let new_value = Box::new(Expr::new(ty, line));
        let ty = ExprType::DotAssign {
            inst: Box::new(inst),
            property: field_name,
            new_value,
        };
        Ok(ty)
    }

    fn string(&mut self) -> Result<Expr<'a>, ParseErr> {
        let Literal::Str(value) = self.previous().literal else {
            unreachable!();
        };
        let kind = ExprType::Lit(Literal::Str(value));
        Ok(Expr::new(kind, self.previous().line))
    }

    fn grouping(&mut self) -> Result<Expr<'a>, ParseErr> {
        let expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
        Ok(expr)
    }

    fn execute_prefix(&mut self, fn_type: FnType, can_assign: bool) -> Result<Expr<'a>, ParseErr> {
        match fn_type {
            FnType::Grouping => self.grouping(),
            FnType::Array => self.array(),
            FnType::Unary => self.unary(),
            FnType::Number => self.number(),
            FnType::String => self.string(),
            FnType::Literal => self.literal(),
            FnType::Var => self.var(can_assign),
            FnType::This => self.this(),
            _ => unreachable!(),
        }
    }

    fn dot(&mut self, inst: Expr<'a>, can_assign: bool) -> Result<Expr<'a>, ParseErr> {
        self.consume(TokenType::Identifier, "Expected property name after '.'.")?;
        let property = self.previous();
        let line = property.line;

        let ty = if self.matches(TokenType::Equal) && can_assign {
            let value = Box::new(self.expression()?);
            ExprType::DotAssign {
                inst: Box::new(inst),
                property: property.lexeme,
                new_value: value,
            }
        } else if can_assign && self.matches(TokenType::PlusEqual) {
            self.get_assign_shorthand_field(property.lexeme, line, BinaryOp::Add, inst)?
        } else if can_assign && self.matches(TokenType::MinEqual) {
            self.get_assign_shorthand_field(property.lexeme, line, BinaryOp::Sub, inst)?
        } else if can_assign && self.matches(TokenType::MulEqual) {
            self.get_assign_shorthand_field(property.lexeme, line, BinaryOp::Mul, inst)?
        } else if can_assign && self.matches(TokenType::DivEqual) {
            self.get_assign_shorthand_field(property.lexeme, line, BinaryOp::Div, inst)?
        } else {
            ExprType::Dot {
                inst: Box::new(inst),
                property: property.lexeme,
            }
        };

        Ok(Expr::new(ty, property.line))
    }
    fn index(&mut self, arr: Expr<'a>, can_assign: bool) -> Result<Expr<'a>, ParseErr> {
        let index = Box::new(self.expression()?);
        self.consume(TokenType::RightBracket, "Expected ']' after index.")?;

        let arr = Box::new(arr);
        let ty = if can_assign && self.matches(TokenType::Equal) {
            let value = Box::new(self.expression()?);
            ExprType::AssignIndex {
                arr,
                index,
                new_value: value,
            }
        } else {
            ExprType::Index { arr, index }
        };
        let expr = Expr::new(ty, self.previous().line);
        Ok(expr)
    }

    fn cast(&mut self, value: Expr<'a>) -> Result<Expr<'a>, ParseErr> {
        let line = self.previous().line;

        if let Some(target) = self.peek().as_value_type() {
            self.advance();
            let value = Box::new(value);
            let ty = ExprType::Cast { value, target };

            Ok(Expr::new(ty, line))
        } else {
            Err(ParseErr {
                line,
                msg: "Expected type after 'as' keyword.".to_string(),
            })
        }
    }

    fn call(&mut self, name: Expr<'a>) -> Result<Expr<'a>, ParseErr> {
        let mut args = Vec::new();
        while !self.check(TokenType::RightParen) {
            args.push(self.expression()?);

            if !self.matches(TokenType::Comma) {
                break;
            }
        }
        self.consume(
            TokenType::RightParen,
            "Expected ')' after function/constructor call.",
        )?;

        if let ExprType::Var(name) = name.expr {
            let ty = ExprType::Call {
                name,
                args,
                index: None,
            };
            let expr = Expr::new(ty, self.previous().line);
            Ok(expr)
        } else if let ExprType::Dot { inst, property } = name.expr {
            let ty = ExprType::MethodCall {
                inst,
                property,
                args,
            };
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
        can_assign: bool,
    ) -> Result<Expr<'a>, ParseErr> {
        match fn_type {
            FnType::Binary => self.binary(left),
            FnType::Call => self.call(left),
            FnType::Index => self.index(left, can_assign),
            FnType::Dot => self.dot(left, can_assign),
            FnType::Cast => self.cast(left),
            _ => unreachable!(),
        }
    }

    fn get_rule(&mut self, kind: TokenType) -> ParseRule {
        PARSE_RULES[kind as usize]
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), ParseErr> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            Err(ParseErr {
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
        self.peek().ty == kind
    }

    fn advance(&mut self) -> Token<'a> {
        if self.peek().ty != TokenType::Eof {
            self.current_token += 1;
        }
        self.previous()
    }

    fn regress(&mut self) {
        self.current_token -= 1;
    }

    fn peek(&self) -> Token<'a> {
        self.tokens[self.current_token]
    }

    fn previous(&self) -> Token<'a> {
        self.tokens[self.current_token - 1]
    }
}
