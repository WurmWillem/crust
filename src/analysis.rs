use crate::{
    error::print_error,
    expression::{Expr, ExprType},
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
    fn new(line: u32, ty: ErrTy) -> Self {
        Self { ty, line }
    }
}
enum ErrTy {
    InvalidPrefix,
    InvalidInfix,
    OpTypeMismatch(ValueType, Operator, ValueType),
    VarTypeMisMatch(ValueType, ValueType),
}
impl SemanticError {
    fn print(&self) {
        let msg = match self.ty {
            ErrTy::InvalidPrefix => format!("invalid prefix bozo"),
            ErrTy::InvalidInfix => format!("invalid infix bozo"),
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

pub struct Analyser {}
impl Analyser {
    fn new() -> Self {
        Self {}
    }
    pub fn analyse_stmts(stmts: &Vec<Stmt>) -> bool {
        let mut analyser = Analyser::new();
        for stmt in stmts {
            if let Err(err) = analyser.analyse_stmt(stmt) {
                err.print();
                return true;
            }
        }
        false
    }

    fn analyse_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        let line = stmt.line;
        match &stmt.stmt {
            StmtType::Expr(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Var { name: _, value, ty } => {
                let value_ty = self.analyse_expr(value)?;
                if value_ty != *ty {
                    let err_ty = ErrTy::VarTypeMisMatch(*ty, value_ty);
                    return Err(SemanticError::new(line, err_ty));
                }
            }
            StmtType::Println(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Return(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Block(stmts) => {
                for stmt in stmts {
                    self.analyse_stmt(stmt)?;
                }
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
            ExprType::Var(name) => todo!(),
            ExprType::Call { name, args } => todo!(),
            ExprType::Assign { name, value } => todo!(),
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
