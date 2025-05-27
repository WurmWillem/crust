use std::collections::HashMap;

use crate::{
    analysis_types::{get_func_data, FuncData, Operator, SemanticScope, Symbol},
    error::{ErrType, SemanticError},
    expression::{Expr, ExprType},
    parse_types::BinaryOp,
    statement::{Stmt, StmtType},
    token::TokenType,
    value::ValueType,
};

pub struct Analyser<'a> {
    func_data: HashMap<&'a str, FuncData<'a>>,
    symbols: SemanticScope<'a>,
}
impl<'a> Analyser<'a> {
    fn new(func_data: HashMap<&'a str, FuncData<'a>>) -> Self {
        Self {
            func_data,
            symbols: SemanticScope::new(),
        }
    }
    pub fn analyse_stmts(stmts: &Vec<Stmt<'a>>) -> Option<HashMap<&'a str, FuncData<'a>>> {
        let func_data = get_func_data(stmts);
        let mut analyser = Analyser::new(func_data);

        for stmt in stmts {
            if let Err(err) = analyser.analyse_stmt(stmt) {
                err.print();
                return None;
            }
        }
        Some(analyser.func_data)
    }

    fn analyse_stmt(&mut self, stmt: &Stmt<'a>) -> Result<(), SemanticError> {
        let line = stmt.line;
        match &stmt.stmt {
            StmtType::Expr(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Var { name, value, ty } => {
                let value_ty = self.analyse_expr(value)?;
                if value_ty != *ty {
                    let err_ty = ErrType::TypeMismatch(*ty, value_ty);
                    return Err(SemanticError::new(line, err_ty));
                }
                self.symbols.declare(Symbol::new(name, *ty), line)?;
            }
            StmtType::Println(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Return(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Block(stmts) => {
                self.symbols.begin_scope();
                for stmt in stmts {
                    self.analyse_stmt(stmt)?;
                }
                self.symbols.end_scope();
            }
            StmtType::If {
                condition,
                body,
                final_else,
            } => {
                self.analyse_expr(condition)?;
                self.analyse_stmt(body)?;
                if let Some(final_else) = final_else {
                    self.analyse_stmt(final_else)?;
                }
            }
            StmtType::While { condition, body } => {
                self.analyse_expr(condition)?;
                self.analyse_stmt(body)?;
            }
            StmtType::For {
                var,
                condition,
                body,
            } => {
                self.analyse_stmt(var)?;
                self.analyse_expr(condition)?;
                self.analyse_stmt(body)?;
            }
            StmtType::Func {
                name: _,
                parameters,
                body,
                return_ty: _,
            } => {
                self.symbols.begin_scope();
                for (ty, name) in parameters {
                    self.symbols.declare(Symbol::new(name, *ty), line)?;
                }
                for stmt in body {
                    self.analyse_stmt(stmt)?;
                }
                self.symbols.end_scope();
            }
        };
        Ok(())
    }
    fn analyse_expr(&mut self, expr: &Expr<'a>) -> Result<ValueType, SemanticError> {
        let line = expr.line;
        let result = match &expr.expr {
            ExprType::Lit(lit) => lit.as_value_type(),
            ExprType::Var(name) => match self.symbols.resolve(name) {
                Some(symbol) => symbol.ty,
                None => {
                    let ty = ErrType::UndefinedVar(name.to_string());
                    return Err(SemanticError::new(line, ty));
                }
            },
            ExprType::Call { name, args } => {
                let data = match self.func_data.remove(name) {
                    Some(data) => data,
                    None => {
                        let ty = ErrType::UndefinedFunc(name.to_string());
                        return Err(SemanticError::new(line, ty));
                    }
                };
                if args.len() != data.parameters.len() {
                    let err_ty = ErrType::IncorrectArity(
                        name.to_string(),
                        args.len() as u8,
                        data.parameters.len() as u8,
                    );
                    return Err(SemanticError::new(line, err_ty));
                }
                for (i, arg) in args.iter().enumerate() {
                    let arg_ty = self.analyse_expr(arg)?;
                    if arg_ty != data.parameters[i].0 {
                        let err_ty = ErrType::TypeMismatch(data.parameters[i].0, arg_ty);
                        return Err(SemanticError::new(line, err_ty));
                    }
                }

                let return_ty = data.return_ty;
                self.func_data.insert(name, data);
                return_ty
            }
            ExprType::Assign { name, value } => match self.symbols.resolve(name) {
                Some(symbol) => {
                    let value_ty = self.analyse_expr(value)?;
                    if symbol.ty != value_ty {
                        let err_ty = ErrType::TypeMismatch(symbol.ty, value_ty);
                        return Err(SemanticError::new(line, err_ty));
                    }
                    symbol.ty
                }
                None => {
                    let ty = ErrType::UndefinedVar(name.to_string());
                    return Err(SemanticError::new(line, ty));
                }
            },
            ExprType::Unary { prefix, value } => {
                let value_ty = self.analyse_expr(value)?;
                match prefix {
                    TokenType::Minus => {
                        if value_ty != ValueType::Num {
                            let err_ty =
                                ErrType::OpTypeMismatch(ValueType::Num, Operator::Minus, value_ty);
                            return Err(SemanticError::new(line, err_ty));
                        }
                        value_ty
                    }
                    TokenType::Bang => {
                        if value_ty != ValueType::Bool {
                            let err_ty =
                                ErrType::OpTypeMismatch(ValueType::Bool, Operator::Bang, value_ty);
                            return Err(SemanticError::new(line, err_ty));
                        }
                        value_ty
                    }
                    _ => return Err(SemanticError::new(line, ErrType::InvalidPrefix)),
                }
            }
            ExprType::Binary { left, op, right } => {
                let left_ty = self.analyse_expr(left)?;
                let right_ty = self.analyse_expr(right)?;
                if left_ty != right_ty {
                    let op = op.to_operator();
                    let err_ty = ErrType::OpTypeMismatch(left_ty, op, right_ty);
                    return Err(SemanticError::new(line, err_ty));
                }

                use BinaryOp as BO;
                let x = match op {
                    BO::Add => left_ty == ValueType::Num || left_ty == ValueType::Str,
                    BO::Sub | BO::Mul | BO::Div => left_ty == ValueType::Num,
                    BO::Equal | BO::NotEqual => true,
                    BO::Less | BO::LessEqual | BO::Greater | BO::GreaterEqual => {
                        left_ty == ValueType::Num
                    }
                    BO::And | BO::Or => left_ty == ValueType::Bool,
                };

                if x {
                    left_ty
                } else {
                    return Err(SemanticError::new(line, ErrType::InvalidInfix));
                }
            }
        };
        Ok(result)
    }
}
