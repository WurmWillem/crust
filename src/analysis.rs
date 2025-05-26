use crate::{
    error::print_error,
    expression::{Expr, ExprType},
    func_compiler::FuncCompilerStack,
    parse_types::{BinaryOp, Operator},
    statement::{Stmt, StmtType},
    token::TokenType,
    value::ValueType,
};

pub struct SemanticError {
    ty: ErrTy,
    line: u32,
}
impl SemanticError {
    pub fn new(line: u32, ty: ErrTy) -> Self {
        Self { ty, line }
    }
}
pub enum ErrTy {
    InvalidPrefix,
    InvalidInfix,
    TooManyLocals,
    UndefinedVar(String),
    OpTypeMismatch(ValueType, Operator, ValueType),
    VarTypeMisMatch(ValueType, ValueType),
}
impl SemanticError {
    fn print(&self) {
        let msg = match &self.ty {
            ErrTy::InvalidPrefix => format!("invalid prefix."),
            ErrTy::InvalidInfix => format!("invalid infix."),
            ErrTy::UndefinedVar(name) => {
                format!("Variable '{}' has not been defined in this scope.", &name)
            }
            ErrTy::TooManyLocals => format!("invalid prefix bozo"),
            ErrTy::OpTypeMismatch(expected, op, found) => {
                format!(
                    "Operator '{}' Expects type '{}', but found type '{}'.",
                    op, expected, found
                )
            }
            ErrTy::VarTypeMisMatch(expected, found) => {
                format!(
                    "Variable was given type '{}' but found type '{}'.",
                    expected, found
                )
            }
        };
        print_error(self.line, &msg);
    }
}

pub struct Analyser<'a> {
    comps: FuncCompilerStack<'a>,
}
impl<'a> Analyser<'a> {
    fn new() -> Self {
        Self {
            comps: FuncCompilerStack::new(),
        }
    }
    pub fn analyse_stmts(stmts: &Vec<Stmt<'a>>) -> Option<FuncCompilerStack<'a>> {
        let mut analyser = Analyser::new();
        for stmt in stmts {
            if let Err(err) = analyser.analyse_stmt(stmt) {
                err.print();
                return None;
            }
        }
        Some(analyser.comps)
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
                    let err_ty = ErrTy::VarTypeMisMatch(*ty, value_ty);
                    return Err(SemanticError::new(line, err_ty));
                }
                self.comps.add_local(name, *ty, line)?;
            }
            StmtType::Println(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Return(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Block(stmts) => {
                self.comps.begin_scope();
                for stmt in stmts {
                    self.analyse_stmt(stmt)?;
                }
                self.comps.end_scope();
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
                parameters: _,
                body,
                return_ty: _,
            } => {
                self.analyse_stmt(body)?;
            }
        };
        Ok(())
    }
    fn analyse_expr(&mut self, expr: &Expr) -> Result<ValueType, SemanticError> {
        let line = expr.line;
        let result = match &expr.expr {
            ExprType::Lit(lit) => lit.as_value_type(),
            ExprType::Var(name) => match self.comps.resolve_local(name) {
                Some((_, ty)) => ty,
                None => {
                    let ty = ErrTy::UndefinedVar(name.to_string());
                    return Err(SemanticError::new(line, ty));
                }
            },
            ExprType::Call { name, args } => todo!(),
            ExprType::Assign { name, value } => match self.comps.resolve_local(name) {
                Some((_, ty)) => {
                    let value_ty = self.analyse_expr(value)?;
                    if ty != value_ty {
                        let err_ty = ErrTy::VarTypeMisMatch(ty, value_ty);
                        return Err(SemanticError::new(line, err_ty));
                    }
                    ty
                }
                None => {
                    let ty = ErrTy::UndefinedVar(name.to_string());
                    return Err(SemanticError::new(line, ty));
                }
            },
            ExprType::Unary { prefix, value } => {
                let value_ty = self.analyse_expr(value)?;
                match prefix {
                    TokenType::Minus => {
                        if value_ty != ValueType::Num {
                            let err_ty =
                                ErrTy::OpTypeMismatch(ValueType::Num, Operator::Minus, value_ty);
                            return Err(SemanticError::new(line, err_ty));
                        }
                        value_ty
                    }
                    TokenType::Bang => {
                        if value_ty != ValueType::Bool {
                            let err_ty =
                                ErrTy::OpTypeMismatch(ValueType::Bool, Operator::Bang, value_ty);
                            return Err(SemanticError::new(line, err_ty));
                        }
                        value_ty
                    }
                    _ => return Err(SemanticError::new(line, ErrTy::InvalidPrefix)),
                }
            }
            ExprType::Binary { left, op, right } => {
                let left_ty = self.analyse_expr(left)?;
                let right_ty = self.analyse_expr(right)?;
                if left_ty != right_ty {
                    let op = op.to_operator();
                    let err_ty = ErrTy::OpTypeMismatch(left_ty, op, right_ty);
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
                    return Err(SemanticError::new(line, ErrTy::InvalidInfix));
                }
            }
        };
        Ok(result)
    }
}
