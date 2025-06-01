use std::collections::HashMap;

use crate::{
    analysis_types::{
        get_nat_func_hash, FuncData, FuncHash, NatFuncHash, Operator, SemanticScope, StructData,
        StructHash, Symbol,
    },
    error::{SemErrType, SemanticErr},
    expression::{Expr, ExprType},
    parse_types::BinaryOp,
    statement::{Stmt, StmtType},
    token::TokenType,
    value::ValueType,
};

pub struct Analyser<'a> {
    // TODO: make it illegal to define a function inside a different function
    funcs: FuncHash<'a>,
    nat_funcs: NatFuncHash<'a>,
    structs: StructHash<'a>,
    symbols: SemanticScope<'a>,
    current_return_ty: ValueType,
}
impl<'a> Analyser<'a> {
    fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            nat_funcs: HashMap::new(),
            symbols: SemanticScope::new(),
            current_return_ty: ValueType::None,
            structs: HashMap::new(),
        }
    }
    pub fn analyse_stmts(
        stmts: &mut Vec<Stmt<'a>>,
    ) -> Option<(FuncHash<'a>, NatFuncHash<'a>, StructHash<'a>)> {
        let mut analyser = Analyser::new();
        if let Err(err) = analyser.init_type_data(stmts) {
            err.print();
            return None;
        }

        for stmt in stmts {
            if let Err(err) = analyser.analyse_stmt(stmt) {
                err.print();
                return None;
            }
        }

        Some((analyser.funcs, analyser.nat_funcs, analyser.structs))
    }

    fn init_type_data(&mut self, stmts: &Vec<Stmt<'a>>) -> Result<(), SemanticErr> {
        self.nat_funcs = get_nat_func_hash();

        for stmt in stmts {
            let line = stmt.line;
            if let StmtType::Func {
                name,
                parameters,
                body,
                return_ty,
            } = &stmt.stmt
            {
                let func_data = FuncData {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    return_ty: return_ty.clone(),
                    line: stmt.line,
                };

                if self.funcs.insert(*name, func_data).is_some() {
                    let err_ty = SemErrType::AlreadyDefinedFunc(name.to_string());
                    return Err(SemanticErr::new(line, err_ty));
                }
            }
            if let StmtType::Struct { name, fields } = &stmt.stmt {
                let fields = fields.clone();
                let struct_data = StructData { fields };

                if self.structs.insert(*name, struct_data).is_some() {
                    let err_ty = SemErrType::AlreadyDefinedStruct(name.to_string());
                    return Err(SemanticErr::new(line, err_ty));
                }
            }
        }
        Ok(())
    }

    fn analyse_stmt(&mut self, stmt: &mut Stmt<'a>) -> Result<(), SemanticErr> {
        let line = stmt.line;
        match &mut stmt.stmt {
            StmtType::Expr(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Var { name, value, ty } => {
                let value_ty = self.analyse_expr(value)?;
                if value_ty != *ty {
                    // TODO: uncomment this
                    // let err_ty = SemErrType::TypeMismatch(ty.clone(), value_ty);
                    // return Err(SemanticErr::new(line, err_ty));
                }
                self.symbols.declare(Symbol::new(name, ty.clone()), line)?;
            }
            StmtType::Println(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Return(expr) => {
                let return_ty = self.analyse_expr(expr)?;

                if return_ty != self.current_return_ty && return_ty != ValueType::Null {
                    let err_ty =
                        SemErrType::IncorrectReturnTy(self.current_return_ty.clone(), return_ty);
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
                self.symbols.begin_scope();
                self.analyse_stmt(var)?;
                self.analyse_expr(condition)?;
                self.analyse_stmt(body)?;
                self.symbols.end_scope();
            }
            StmtType::Func {
                name: _,
                parameters,
                body,
                return_ty,
            } => {
                let prev_return_ty = self.current_return_ty.clone();
                self.current_return_ty = return_ty.clone();

                self.symbols.begin_scope();
                for (ty, name) in parameters {
                    self.symbols.declare(Symbol::new(name, ty.clone()), line)?;
                }
                for stmt in body {
                    self.analyse_stmt(stmt)?;
                }
                self.symbols.end_scope();
                self.current_return_ty = prev_return_ty;
            }
            StmtType::Break => (),
            StmtType::Continue => (),
            StmtType::Struct { name, fields } => {
                // TODO: this
            }
        };
        Ok(())
    }

    fn analyse_expr(&mut self, expr: &mut Expr<'a>) -> Result<ValueType, SemanticErr> {
        let line = expr.line;
        let result = match &mut expr.expr {
            ExprType::Lit(lit) => lit.as_value_type(),
            ExprType::Var(name) => match self.symbols.resolve(name) {
                Some(symbol) => symbol.ty,
                None => {
                    let ty = SemErrType::UndefinedVar(name.to_string());
                    return Err(SemanticErr::new(line, ty));
                }
            },
            ExprType::Call { name, args } => {
                if let Some(_) = self.structs.get(name) {
                    return Ok(ValueType::Struct(name.to_string()));
                }

                let (return_ty, parameters) = self.get_called_func_data(name, line)?;
                if args.len() != parameters.len() {
                    let err_ty = SemErrType::IncorrectArity(
                        name.to_string(),
                        args.len() as u8,
                        parameters.len() as u8,
                    );
                    return Err(SemanticErr::new(line, err_ty));
                }
                for (i, arg) in args.iter_mut().enumerate() {
                    let arg_ty = self.analyse_expr(arg)?;

                    let param_ty = &parameters[i];

                    let is_exact_match = arg_ty == *param_ty;
                    let is_any = *param_ty == ValueType::Any;
                    let is_array_match = matches!(param_ty, ValueType::Arr(inner) if **inner == ValueType::Any)
                        && matches!(arg_ty, ValueType::Arr(_));

                    if !is_exact_match && !is_any && !is_array_match {
                        let err_ty = SemErrType::TypeMismatch(param_ty.clone(), arg_ty);
                        return Err(SemanticErr::new(line, err_ty));
                    }
                }

                return_ty
            }
            ExprType::Assign { name, value } => match self.symbols.resolve(name) {
                Some(symbol) => {
                    let value_ty = self.analyse_expr(value)?;
                    if symbol.ty != value_ty && symbol.ty != ValueType::Any {
                        let err_ty = SemErrType::TypeMismatch(symbol.ty, value_ty);
                        return Err(SemanticErr::new(line, err_ty));
                    }
                    symbol.ty
                }
                None => {
                    let ty = SemErrType::UndefinedVar(name.to_string());
                    return Err(SemanticErr::new(line, ty));
                }
            },
            ExprType::Unary { prefix, value } => {
                let value_ty = self.analyse_expr(value)?;
                match prefix {
                    TokenType::Minus => {
                        if value_ty != ValueType::Num {
                            let err_ty = SemErrType::OpTypeMismatch(
                                ValueType::Num,
                                Operator::Minus,
                                value_ty,
                            );
                            return Err(SemanticErr::new(line, err_ty));
                        }
                        value_ty
                    }
                    TokenType::Bang => {
                        if value_ty != ValueType::Bool {
                            let err_ty = SemErrType::OpTypeMismatch(
                                ValueType::Bool,
                                Operator::Bang,
                                value_ty,
                            );
                            return Err(SemanticErr::new(line, err_ty));
                        }
                        value_ty
                    }
                    _ => return Err(SemanticErr::new(line, SemErrType::InvalidPrefix)),
                }
            }
            ExprType::Binary { left, op, right } => {
                let left_ty = self.analyse_expr(left)?;
                let right_ty = self.analyse_expr(right)?;
                if left_ty != right_ty {
                    let op = op.to_operator();
                    let err_ty = SemErrType::OpTypeMismatch(left_ty, op, right_ty);
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
                    return Err(SemanticErr::new(line, SemErrType::InvalidInfix));
                }
            }
            ExprType::Array(values) => {
                let el_ty = self.analyse_expr(&mut values[0])?;

                for el in values.iter_mut().skip(1) {
                    let next_el_ty = self.analyse_expr(el)?;

                    if next_el_ty != el_ty {
                        let err_ty = SemErrType::ArrElTypeMismatch(el_ty, next_el_ty);
                        return Err(SemanticErr::new(line, err_ty));
                    }
                }
                ValueType::Arr(Box::new(el_ty))
            }
            ExprType::Index { arr, index: _ } => {
                let arr = self.analyse_expr(arr)?;
                match arr {
                    ValueType::Arr(ty) => *ty,
                    _ => {
                        let ty = SemErrType::IndexNonArr(arr);
                        return Err(SemanticErr::new(line, ty));
                    }
                }
            }
            ExprType::AssignIndex {
                arr,
                index: _,
                value,
            } => {
                let arr = self.analyse_expr(arr)?;
                match arr {
                    ValueType::Arr(ty) => {
                        let value_ty = self.analyse_expr(value)?;
                        if value_ty != *ty {
                            let ty = SemErrType::AssignArrTypeMismatch(*ty, value_ty);
                            return Err(SemanticErr::new(line, ty));
                        }
                        *ty
                    }
                    _ => {
                        let ty = SemErrType::IndexNonArr(arr);
                        return Err(SemanticErr::new(line, ty));
                    }
                }
            }
            ExprType::Dot { inst, property } => {
                let inst_ty = self.analyse_expr(inst)?;
                dbg!(&inst);
                let ValueType::Struct(name) = inst_ty else {
                    unreachable!()
                };
                let Some(data) = self.structs.get(&name as &str) else {
                    unreachable!()
                };
                let x = data.fields.iter().find(|f| f.1 == *property).unwrap();

                let index = data
                    .fields
                    .iter()
                    .position(|(_, field_name)| field_name == property)
                    .unwrap() as u8;

                expr.expr = ExprType::DotResolved {
                    inst: inst.clone(),
                    index,
                };
                // todo!()
                // dbg!(inst_ty);
                data.fields[index as usize].0.clone()
                // x.0.clone()
            }
            ExprType::DotResolved { inst, index } => todo!(),
        };
        Ok(result)
    }
    // fn get_struct_data(
    //     &mut self,
    //     name: &'a str,
    //     line: u32,
    // ) -> Result<(ValueType, Vec<ValueType>), SemanticErr> {
    //     if let Some(data) = self.struct_data.remove(name) {
    //         data.fields
    //         let parameters = data.parameters.iter().map(|p| p.0.clone()).collect();
    //         let return_ty = data.return_ty.clone();
    //
    //         self.func_data.insert(name, data);
    //
    //         return Ok((return_ty, parameters));
    //     };
    // }

    fn get_called_func_data(
        &mut self,
        name: &'a str,
        line: u32,
    ) -> Result<(ValueType, Vec<ValueType>), SemanticErr> {
        if let Some(data) = self.funcs.remove(name) {
            let parameters = data.parameters.iter().map(|p| p.0.clone()).collect();
            let return_ty = data.return_ty.clone();

            self.funcs.insert(name, data);

            return Ok((return_ty, parameters));
        };
        if let Some(data) = self.nat_funcs.remove(name) {
            let parameters = data.parameters.clone();
            let return_ty = data.return_ty.clone();

            self.nat_funcs.insert(name, data);

            return Ok((return_ty, parameters));
        };
        let ty = SemErrType::UndefinedFunc(name.to_string());
        Err(SemanticErr::new(line, ty))
    }
}
