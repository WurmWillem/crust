use crate::{
    expression::{Expr, ExprType},
    parse_types::BinaryOp,
    statement::{Stmt, StmtType},
    token::TokenType,
    value::ValueType,
};

enum SemanticError {
    InvalidPrefix,
    InvalidInfix,
}
impl SemanticError {
    fn print(&self) {
        match self {
            SemanticError::InvalidPrefix => println!("invalid prefix bozo"),
            SemanticError::InvalidInfix => println!("invalid infix bozo"),
        }
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

    fn analyse_stmt(&mut self, stmt: &Stmt) -> Result<ValueType, SemanticError> {
        //let line = stmt.line;
        match &stmt.stmt {
            StmtType::Expr(expr) => self.analyse_expr(expr),
            StmtType::Var { name, value, ty } => todo!(),
            StmtType::Println(_) => todo!(),
            StmtType::Return(_) => todo!(),
            StmtType::Block(_) => todo!(),
            StmtType::If {
                condition,
                body,
                final_else,
            } => todo!(),
            StmtType::While { condition, body } => todo!(),
            StmtType::For {
                var,
                condition,
                body,
            } => todo!(),
            StmtType::Func {
                name,
                parameters,
                body,
                return_ty,
            } => todo!(),
        }
    }
    fn analyse_expr(&mut self, expr: &Expr) -> Result<ValueType, SemanticError> {
        let result = match &expr.expr {
            ExprType::Lit(lit) => lit.as_value_type(),
            ExprType::Var(name) => todo!(),
            ExprType::Call { name, args } => todo!(),
            ExprType::Assign { name, value } => todo!(),
            ExprType::Unary { prefix, value } => {
                let value_ty = self.analyse_expr(value)?;
                match prefix {
                    TokenType::Minus | TokenType::Bang => return Ok(value_ty),
                    _ => return Err(SemanticError::InvalidPrefix),
                }
            }
            ExprType::Binary { left, op, right } => {
                let left_ty = self.analyse_expr(left)?;
                let right_ty = self.analyse_expr(right)?;
                if left_ty != right_ty {
                    return Err(SemanticError::InvalidInfix);
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
                    return Err(SemanticError::InvalidInfix);
                }
            }
        };
        Ok(result)
    }
}
