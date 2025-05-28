use crate::{
    analysis_types::{get_func_data, FuncHash, NatFuncHash, Operator, SemanticScope, Symbol},
    error::{print_error, ErrType, SemanticErr},
    expression::{Expr, ExprType},
    parse_types::BinaryOp,
    statement::{Stmt, StmtType},
    token::TokenType,
    value::ValueType,
};

pub struct Analyser<'a> {
    func_data: FuncHash<'a>,
    nat_func_data: NatFuncHash<'a>,
    symbols: SemanticScope<'a>,
    current_return_ty: ValueType,
}
impl<'a> Analyser<'a> {
    fn new(func_data: FuncHash<'a>, nat_func_data: NatFuncHash<'a>) -> Self {
        Self {
            func_data,
            nat_func_data,
            symbols: SemanticScope::new(),
            current_return_ty: ValueType::None,
        }
    }
    pub fn analyse_stmts(stmts: &Vec<Stmt<'a>>) -> Option<(FuncHash<'a>, NatFuncHash<'a>)> {
        let (func_data, nat_func_data) = match get_func_data(stmts) {
            Some(data) => data,
            None => {
                print_error(0, "Function with the same name has already been defined.");
                return None;
            }
        };
        let mut analyser = Analyser::new(func_data, nat_func_data);

        //for stmt in stmts {
        //    if let Err(err) = analyser.analyse_stmt(stmt) {
        //        err.print();
        //        return None;
        //    }
        //}

        Some((analyser.func_data, analyser.nat_func_data))
    }

    fn analyse_stmt(&mut self, stmt: &Stmt<'a>) -> Result<(), SemanticErr> {
        let line = stmt.line;
        match &stmt.stmt {
            StmtType::Expr(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Var { name, value, ty } => {
                let value_ty = self.analyse_expr(value)?;
                if value_ty != *ty {
                    let err_ty = ErrType::TypeMismatch(*ty, value_ty);
                    return Err(SemanticErr::new(line, err_ty));
                }
                self.symbols.declare(Symbol::new(name, *ty), line)?;
            }
            StmtType::Println(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Return(expr) => {
                let return_ty = self.analyse_expr(expr)?;

                if return_ty != self.current_return_ty && return_ty != ValueType::Null {
                    let err_ty = ErrType::IncorrectReturnTy(self.current_return_ty, return_ty);
                    return Err(SemanticErr::new(line, err_ty));
                }
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
                return_ty,
            } => {
                let prev_return_ty = self.current_return_ty;
                self.current_return_ty = *return_ty;

                self.symbols.begin_scope();
                for (ty, name) in parameters {
                    self.symbols.declare(Symbol::new(name, *ty), line)?;
                }
                for stmt in body {
                    self.analyse_stmt(stmt)?;
                }
                self.symbols.end_scope();
                self.current_return_ty = prev_return_ty;
            }
            StmtType::Break => (),
            StmtType::Continue => (),
        };
        Ok(())
    }
    fn get_called_func_data(
        &mut self,
        name: &'a str,
        line: u32,
    ) -> Result<(ValueType, Vec<ValueType>), SemanticErr> {
        if let Some(data) = self.func_data.remove(name) {
            let parameters = data.parameters.iter().map(|p| p.0).collect();
            let return_ty = data.return_ty;

            self.func_data.insert(name, data);

            return Ok((return_ty, parameters));
        };
        if let Some(data) = self.nat_func_data.remove(name) {
            let parameters = data.parameters.clone();
            let return_ty = data.return_ty;

            self.nat_func_data.insert(name, data);

            return Ok((return_ty, parameters));
        };
        let ty = ErrType::UndefinedFunc(name.to_string());
        Err(SemanticErr::new(line, ty))
    }
    fn analyse_expr(&mut self, expr: &Expr<'a>) -> Result<ValueType, SemanticErr> {
        let line = expr.line;
        let result = match &expr.expr {
            ExprType::Lit(lit) => lit.as_value_type(),
            ExprType::Var(name) => match self.symbols.resolve(name) {
                Some(symbol) => symbol.ty,
                None => {
                    let ty = ErrType::UndefinedVar(name.to_string());
                    return Err(SemanticErr::new(line, ty));
                }
            },
            ExprType::Call { name, args } => {
                let (return_ty, parameters) = self.get_called_func_data(name, line)?;
                if args.len() != parameters.len() {
                    let err_ty = ErrType::IncorrectArity(
                        name.to_string(),
                        args.len() as u8,
                        parameters.len() as u8,
                    );
                    return Err(SemanticErr::new(line, err_ty));
                }
                for (i, arg) in args.iter().enumerate() {
                    let arg_ty = self.analyse_expr(arg)?;
                    if arg_ty != parameters[i] && parameters[i] != ValueType::Any {
                        let err_ty = ErrType::TypeMismatch(parameters[i], arg_ty);
                        return Err(SemanticErr::new(line, err_ty));
                    }
                }

                return_ty
            }
            ExprType::Assign { name, value } => match self.symbols.resolve(name) {
                Some(symbol) => {
                    let value_ty = self.analyse_expr(value)?;
                    if symbol.ty != value_ty && symbol.ty != ValueType::Any {
                        let err_ty = ErrType::TypeMismatch(symbol.ty, value_ty);
                        return Err(SemanticErr::new(line, err_ty));
                    }
                    symbol.ty
                }
                None => {
                    let ty = ErrType::UndefinedVar(name.to_string());
                    return Err(SemanticErr::new(line, ty));
                }
            },
            ExprType::Unary { prefix, value } => {
                let value_ty = self.analyse_expr(value)?;
                match prefix {
                    TokenType::Minus => {
                        if value_ty != ValueType::Num {
                            let err_ty =
                                ErrType::OpTypeMismatch(ValueType::Num, Operator::Minus, value_ty);
                            return Err(SemanticErr::new(line, err_ty));
                        }
                        value_ty
                    }
                    TokenType::Bang => {
                        if value_ty != ValueType::Bool {
                            let err_ty =
                                ErrType::OpTypeMismatch(ValueType::Bool, Operator::Bang, value_ty);
                            return Err(SemanticErr::new(line, err_ty));
                        }
                        value_ty
                    }
                    _ => return Err(SemanticErr::new(line, ErrType::InvalidPrefix)),
                }
            }
            ExprType::Binary { left, op, right } => {
                let left_ty = self.analyse_expr(left)?;
                let right_ty = self.analyse_expr(right)?;
                if left_ty != right_ty {
                    let op = op.to_operator();
                    let err_ty = ErrType::OpTypeMismatch(left_ty, op, right_ty);
                    return Err(SemanticErr::new(line, err_ty));
                }

                use BinaryOp as BO;
                let x = match op {
                    BO::Add => left_ty == ValueType::Num || left_ty == ValueType::Str,
                    BO::Sub | BO::Mul | BO::Div => left_ty == ValueType::Num,
                    BO::Equal | BO::NotEqual => return Ok(ValueType::Bool),
                    BO::Less | BO::LessEqual | BO::Greater | BO::GreaterEqual => {
                        if left_ty == ValueType::Num {
                            return Ok(ValueType::Bool);
                        }
                        false
                    }
                    BO::And | BO::Or => left_ty == ValueType::Bool,
                };

                if x {
                    left_ty
                } else {
                    return Err(SemanticErr::new(line, ErrType::InvalidInfix));
                }
            }
            ExprType::Array(_) => ValueType::Arr,
            ExprType::Index { name, index } => todo!(),
        };
        Ok(result)
    }
}
