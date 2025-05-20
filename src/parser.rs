use colored::Colorize;

use crate::{
    compiler_types::*,
    error::{print_error, ParseError},
    token::{Literal, Token, TokenType},
    value::ValueType,
};

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    Greater,
    Equal,
}

#[derive(Debug)]
pub enum Expr<'a> {
    Lit(Literal<'a>),
    Variable(String),
    Unary {
        prefix: TokenType,
        right: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        op: BinaryOp,
        right: Box<Expr<'a>>,
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
        let mut result = Expr::Lit(Literal::Null);
        while !parser.check(TokenType::Eof) {
            let parsed = parser.expression() ;
            result = match parsed {
                Ok(result) => result,
                Err(err) => {
                    print_error(err.line, &err.msg);

                    had_error = true;
                    Expr::Lit(Literal::Null)
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

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Expr<'a>, ParseError> {
        self.advance();
        let kind = self.previous().kind;
        let prefix = self.get_rule(kind).prefix;

        if prefix == FnType::Empty {
            let msg = "Expected expression.";
            let err = ParseError::new(self.peek().line, msg);
            return Err(err);
        }
        let can_assign = precedence <= Precedence::Assignment;
        dbg!(prefix);
        self.number()
        // self.execute_fn_type(prefix, can_assign)?;

        // while precedence <= self.get_rule(self.peek().kind).precedence {
        //     self.advance();
        //     let infix = self.get_rule(self.previous().kind).infix;
        //     self.execute_fn_type(infix, can_assign)?;
        // }
        // Ok(())
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParseError> {
        // self.number()
        self.parse_precedence(Precedence::Assignment)
    }

    fn var_or_func(&mut self, can_assign: bool) -> Result<(), ParseError> {
        todo!()
    }

    fn resolve_local(&mut self, name: &str, can_assign: bool) -> Result<bool, ParseError> {
        todo!()
    }

    fn string(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn number(&mut self) -> Result<Expr<'a>, ParseError> {
        let Literal::Num(value) = self.previous().literal else {
            unreachable!();
        };
        Ok(Expr::Lit(Literal::Num(value)))
    }

    fn binary(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn unary(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn grouping(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn literal(&mut self) {
        match self.previous().kind {
            TokenType::True => {}
            TokenType::False => {}
            TokenType::Null => {}
            _ => unreachable!(),
        }
    }

    fn execute_fn_type(&mut self, fn_type: FnType, can_assign: bool) -> Result<(), ParseError> {
        todo!()
        // dbg!(fn_type);
        // match fn_type {
        //     FnType::Grouping => self.grouping(),
        //     FnType::Unary => self.unary(),
        //     FnType::Binary => self.binary(),
        //     FnType::Number => self.number(),
        //     FnType::String => self.string(),
        //     FnType::Variable => self.var_or_func(can_assign),
        //     FnType::Literal => {
        //         self.literal();
        //         Ok(())
        //     }
        //     FnType::Empty => Ok(()),
        //     FnType::Call => unreachable!(),
        // }
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
