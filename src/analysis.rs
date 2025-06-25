use crate::{
    analysis_types::{EnityData, FuncData, Operator, SemanticScope, StructData, Symbol},
    error::{SemErr, SemErrType},
    expression::{Expr, ExprType},
    parse_types::BinaryOp,
    statement::{Stmt, StmtType},
    token::{Literal, TokenType},
    value::ValueType,
};

pub struct Analyser<'a> {
    enities: EnityData<'a>,
    symbols: SemanticScope<'a>,
    current_return_ty: ValueType,
    current_use_self: bool,
    return_stmt_found: bool,
    current_struct: Option<&'a str>,
}
impl<'a> Analyser<'a> {
    fn new() -> Self {
        Self {
            symbols: SemanticScope::new(),
            current_return_ty: ValueType::None,
            enities: EnityData::new(),
            current_struct: None,
            return_stmt_found: false,
            current_use_self: false,
        }
    }
    pub fn analyse_stmts(stmts: &mut Vec<Stmt<'a>>) -> Option<EnityData<'a>> {
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

        Some(analyser.enities)
    }

    fn init_type_data(&mut self, stmts: &mut Vec<Stmt<'a>>) -> Result<(), SemErr> {
        let (nat_funcs, nat_structs) = crate::native::register();
        self.enities.nat_funcs = nat_funcs;
        self.enities.nat_structs = nat_structs;

        for stmt in stmts {
            let line = stmt.line;
            if let StmtType::Func {
                name,
                parameters,
                body: _,
                return_ty,
                use_self,
            } = &stmt.stmt
            {
                let func_data = FuncData {
                    parameters: parameters.clone(),
                    body: vec![],
                    return_ty: return_ty.clone(),
                    line: stmt.line,
                    use_self: *use_self,
                };

                if self.enities.funcs.insert(*name, func_data).is_some() {
                    let err_ty = SemErrType::AlreadyDefinedFunc(name.to_string());
                    return Err(SemErr::new(line, err_ty));
                }
            } else if let StmtType::Struct {
                name,
                fields,
                methods,
            } = &mut stmt.stmt
            {
                if self.current_struct.is_some() {
                    // TODO: this doesn't work for some reason
                    let ty = SemErrType::StructDefInFunc(name.to_string());
                    return Err(SemErr::new(line, ty));
                }
                self.current_struct = Some(name);
                let struct_data = StructData::new(fields.clone());
                let mut method_data = vec![];

                if self.enities.structs.insert(*name, struct_data).is_some() {
                    let err_ty = SemErrType::AlreadyDefinedStruct(name.to_string());
                    return Err(SemErr::new(line, err_ty));
                }

                for method in methods.iter() {
                    if let StmtType::Func {
                        name,
                        parameters,
                        body: _,
                        return_ty,
                        use_self,
                    } = &method.stmt
                    {
                        let func_data = FuncData {
                            parameters: parameters.clone(),
                            body: vec![],
                            return_ty: return_ty.clone(),
                            line: stmt.line,
                            use_self: *use_self,
                        };
                        method_data.push((*name, func_data));
                    } else {
                        unreachable!()
                    }
                }

                self.enities.structs.get_mut(name).unwrap().methods = method_data.clone();

                for (i, method) in methods.iter_mut().enumerate() {
                    self.analyse_stmt(method)?;

                    if let StmtType::Func { body, .. } = &method.stmt {
                        method_data[i].1.body = body.clone();
                    } else {
                        unreachable!()
                    }
                }

                self.enities.structs.get_mut(name).unwrap().methods = method_data;
                self.current_struct = None;
            }
        }

        if !self.enities.funcs.contains_key("main") {
            let err_ty = SemErrType::NoMainFunc;
            return Err(SemErr::new(0, err_ty));
        }
        Ok(())
    }

    fn analyse_stmt(&mut self, stmt: &mut Stmt<'a>) -> Result<(), SemErr> {
        let line = stmt.line;
        match &mut stmt.stmt {
            StmtType::Expr(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Var { name, value, ty } => {
                if let ValueType::Struct(name) = ty {
                    if !self.enities.structs.contains_key(name as &str)
                        && !self.enities.nat_structs.contains_key(name as &str)
                    {
                        let err = SemErrType::UndefinedStruct(name.clone());
                        return Err(SemErr::new(line, err));
                    }
                }

                let value_ty = self.analyse_expr(value)?;
                if value_ty != *ty
                    && value_ty != ValueType::Null
                    && !try_coerce(&mut value.expr, ty)
                {
                    let err_ty = SemErrType::VarDeclTypeMismatch(ty.clone(), value_ty);
                    return Err(SemErr::new(line, err_ty));
                }

                self.symbols.declare(Symbol::new(name, ty.clone()), line)?;
            }
            StmtType::Println(expr) => {
                self.analyse_expr(expr)?;
            }
            StmtType::Return(expr) => {
                self.return_stmt_found = true;
                let return_ty = self.analyse_expr(expr)?;

                if return_ty != self.current_return_ty
                    && return_ty != ValueType::Null
                    && !try_coerce(&mut expr.expr, &self.current_return_ty)
                {
                    let err_ty =
                        SemErrType::IncorrectReturnTy(self.current_return_ty.clone(), return_ty);
                    return Err(SemErr::new(line, err_ty));
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
                name,
                parameters,
                body,
                return_ty,
                use_self,
            } => {
                self.analyse_func_stmt(return_ty.clone(), parameters, line, body, name, *use_self)?;
            }
            StmtType::Break => (),
            StmtType::Continue => (),
            StmtType::Struct { .. } => (),
        };
        Ok(())
    }

    fn analyse_expr(&mut self, expr: &mut Expr<'a>) -> Result<ValueType, SemErr> {
        let line = expr.line;
        let result = match &mut expr.expr {
            ExprType::Lit(lit) => lit.as_value_type(),
            ExprType::Var(name) => match self.symbols.resolve(name) {
                Some(symbol) => symbol.ty,
                None => {
                    let ty = SemErrType::UndefinedVar(name.to_string());
                    return Err(SemErr::new(line, ty));
                }
            },
            ExprType::FuncCall { name, args, index } => {
                if let Some(data) = self.enities.nat_funcs.remove(name) {
                    for (i, func) in data.iter().enumerate() {
                        let parameters = func.parameters.clone();
                        let return_ty = func.return_ty.clone();

                        if let Ok(()) = self.check_if_params_and_args_correspond(
                            args,
                            parameters,
                            name.to_string(),
                            line,
                        ) {
                            self.enities.nat_funcs.insert(name, data);
                            *index = Some(i);
                            return Ok(return_ty);
                        }
                    }
                    self.enities.nat_funcs.insert(name, data);

                    let err_ty = SemErrType::NatParamTypeMismatch(name.to_string());
                    return Err(SemErr::new(line, err_ty));
                };

                let (return_ty, parameters) = self.get_called_func_data(name, line)?;
                self.check_if_params_and_args_correspond(args, parameters, name.to_string(), line)?;
                *index = Some(0);
                return_ty
            }
            ExprType::Assign {
                name,
                new_value: value,
            } => self.analyse_assign(name, value, line)?,
            ExprType::Unary { prefix, value } => self.analyse_unary(value, *prefix, line)?,
            ExprType::Binary { left, op, right } => self.analyse_binary(left, right, *op, line)?,
            ExprType::Array(values) => self.analyse_array_expr(values, line)?,
            ExprType::Index { arr, index: _ } => {
                let arr = self.analyse_expr(arr)?;
                match arr {
                    ValueType::Arr(ty) => *ty,
                    _ => {
                        let ty = SemErrType::IndexNonArr(arr);
                        return Err(SemErr::new(line, ty));
                    }
                }
            }
            ExprType::AssignIndex {
                arr,
                index: _,
                new_value: value,
            } => self.analyse_assign_index(arr, value, line)?,
            ExprType::Dot { inst, property } => {
                let (return_ty, new_expr) = self.analyse_dot(None, inst, line, property)?;
                expr.expr = new_expr;
                return_ty
            }
            ExprType::DotAssign {
                inst,
                property,
                new_value,
            } => {
                let (return_ty, new_expr) =
                    self.analyse_dot(Some(new_value), inst, line, property)?;
                expr.expr = new_expr;
                return_ty
            }
            ExprType::MethodCall {
                inst,
                property,
                args,
            } => {
                let (index, return_ty, use_self) =
                    self.analyse_method_call(inst, property, line, args)?;

                expr.expr = ExprType::MethodCallResolved {
                    inst: inst.clone(),
                    index,
                    args: args.clone(),
                    use_self,
                };
                return_ty
            }
            ExprType::Cast { value, target } => {
                let value_ty = self.analyse_expr(value)?;
                if !value_ty.is_num() || !target.is_num() {
                    let ty = SemErrType::InvalidCast(target.clone(), value_ty);
                    return Err(SemErr::new(line, ty));
                }
                target.clone()
            }
            ExprType::This => unreachable!(),
            ExprType::DotResolved { .. } => unreachable!(),
            ExprType::MethodCallResolved { .. } => unreachable!(),
            ExprType::DotAssignResolved { .. } => unreachable!(),
            ExprType::Colon { inst, property } => todo!(),
        };
        Ok(result)
    }

    fn analyse_func_stmt(
        &mut self,
        return_ty: ValueType,
        parameters: &mut Vec<(ValueType, &'a str)>,
        line: u32,
        body: &mut [Stmt<'a>],
        name: &str,
        use_self: bool,
    ) -> Result<(), SemErr> {
        if self.current_return_ty != ValueType::None {
            let ty = SemErrType::FuncDefInFunc(name.to_string());
            return Err(SemErr::new(line, ty));
        }
        let prev_return_ty = self.current_return_ty.clone();
        let prev_use_self = self.current_use_self;
        self.current_return_ty = return_ty.clone();
        self.current_use_self = use_self;

        self.symbols.begin_scope();

        for (ty, name) in parameters {
            self.symbols.declare(Symbol::new(name, ty.clone()), line)?;
        }
        self.return_stmt_found = false;

        for stmt in body.iter_mut() {
            self.analyse_stmt(stmt)?;
        }

        if let Some(func) = self.enities.funcs.get_mut(name) {
            func.body = body.to_owned();
        }

        if return_ty != ValueType::Null && !self.return_stmt_found {
            let ty = SemErrType::NoReturnTy(name.to_string(), return_ty.clone());
            return Err(SemErr::new(line, ty));
        }

        self.symbols.end_scope();
        self.current_return_ty = prev_return_ty;
        self.current_use_self = prev_use_self;

        Ok(())
    }

    fn analyse_assign(
        &mut self,
        name: &str,
        value: &mut Box<Expr<'a>>,
        line: u32,
    ) -> Result<ValueType, SemErr> {
        match self.symbols.resolve(name) {
            Some(symbol) => {
                let value_ty = self.analyse_expr(value)?;
                if symbol.ty != value_ty
                    && symbol.ty != ValueType::Any
                    && !try_coerce(&mut value.expr, &symbol.ty)
                {
                    let err_ty = SemErrType::VarDeclTypeMismatch(symbol.ty, value_ty);
                    return Err(SemErr::new(line, err_ty));
                }
                Ok(symbol.ty)
            }
            None => {
                let ty = SemErrType::UndefinedVar(name.to_string());
                Err(SemErr::new(line, ty))
            }
        }
    }

    fn analyse_method_call(
        &mut self,
        inst: &mut Box<Expr<'a>>,
        property: &str,
        line: u32,
        args: &mut [Expr<'a>],
    ) -> Result<(u8, ValueType, bool), SemErr> {
        let name = if let ExprType::This = inst.expr {

            let Some(name) = self.current_struct else {
                let ty = SemErrType::SelfOutsideStruct;
                return Err(SemErr::new(line, ty));
            };

            if !self.current_use_self {
                let ty = SemErrType::SelfInMethodWithoutSelfParam;
                return Err(SemErr::new(line, ty));
            }
            name.to_string()
        } else {
            let inst_ty = self.analyse_expr(inst)?;

            let ValueType::Struct(name) = inst_ty else {
                let ty = SemErrType::InvalidTypeMethodAccess(inst_ty);
                return Err(SemErr::new(line, ty));
            };
            name
        };

        for arg in args.iter_mut() {
            self.analyse_expr(arg)?;
        }

        if let Some(data) = self.enities.structs.get(&name as &str) {
            let (index, return_ty, use_self, parameters) =
                data.get_method_data(&name, property, line)?;
            self.check_if_params_and_args_correspond(args, parameters, name, line)?;

            Ok((index, return_ty, use_self))
        } else if let Some(data) = self.enities.nat_structs.get(&name as &str) {
            let (index, return_ty, use_self, parameters) =
                data.get_method_index_and_return_ty(&name, property, line)?;
            self.check_if_params_and_args_correspond(args, parameters, name, line)?;

            Ok((index, return_ty, use_self))
        } else {
            let ty = SemErrType::UndefinedStruct(name);
            Err(SemErr::new(line, ty))
        }
    }

    fn analyse_assign_index(
        &mut self,
        arr: &mut Box<Expr<'a>>,
        value: &mut Box<Expr<'a>>,
        line: u32,
    ) -> Result<ValueType, SemErr> {
        let arr = self.analyse_expr(arr)?;
        Ok(match arr {
            ValueType::Arr(ty) => {
                let value_ty = self.analyse_expr(value)?;
                if value_ty != *ty {
                    let ty = SemErrType::AssignArrTypeMismatch(*ty, value_ty);
                    return Err(SemErr::new(line, ty));
                }
                *ty
            }
            _ => {
                let ty = SemErrType::IndexNonArr(arr);
                return Err(SemErr::new(line, ty));
            }
        })
    }

    fn analyse_array_expr(
        &mut self,
        values: &mut [Expr<'a>],
        line: u32,
    ) -> Result<ValueType, SemErr> {
        if values.is_empty() {
            return Ok(ValueType::Arr(Box::new(ValueType::Any)));
        }
        let el_ty = self.analyse_expr(&mut values[0])?;
        for el in values.iter_mut().skip(1) {
            let next_el_ty = self.analyse_expr(el)?;

            if next_el_ty != el_ty && !try_coerce(&mut el.expr, &el_ty) {
                let err_ty = SemErrType::ArrElTypeMismatch(el_ty, next_el_ty);
                return Err(SemErr::new(line, err_ty));
            }
        }
        Ok(ValueType::Arr(Box::new(el_ty)))
    }

    fn analyse_binary(
        &mut self,
        left: &mut Box<Expr<'a>>,
        right: &mut Box<Expr<'a>>,
        op: BinaryOp,
        line: u32,
    ) -> Result<ValueType, SemErr> {
        let left_ty = self.analyse_expr(left)?;
        let right_ty = self.analyse_expr(right)?;

        if left_ty != right_ty
            && !try_coerce(&mut right.expr, &left_ty)
            && !try_coerce(&mut left.expr, &right_ty)
        {
            let op = op.to_operator();
            let err_ty = SemErrType::OpTypeMismatch(left_ty, op, right_ty);
            return Err(SemErr::new(line, err_ty));
        }

        use BinaryOp as BO;
        let is_valid = match op {
            BO::Add => left_ty.is_num() || left_ty == ValueType::Str,
            BO::Sub | BO::Mul | BO::Div => left_ty.is_num(),
            BO::Equal | BO::NotEqual => return Ok(ValueType::Bool),
            BO::Less | BO::LessEqual | BO::Greater | BO::GreaterEqual => {
                if left_ty.is_num() {
                    return Ok(ValueType::Bool);
                }
                false
            }
            BO::And | BO::Or => left_ty == ValueType::Bool,
        };

        if is_valid {
            Ok(left_ty)
        } else {
            Err(SemErr::new(line, SemErrType::InvalidInfix))
        }
    }

    fn analyse_unary(
        &mut self,
        value: &mut Box<Expr<'a>>,
        prefix: TokenType,
        line: u32,
    ) -> Result<ValueType, SemErr> {
        let value_ty = self.analyse_expr(value)?;

        match prefix {
            TokenType::Minus => {
                if value_ty != ValueType::I64 && value_ty != ValueType::F64 {
                    let err_ty =
                        SemErrType::OpTypeMismatch(ValueType::I64, Operator::Minus, value_ty);
                    return Err(SemErr::new(line, err_ty));
                }
                Ok(value_ty)
            }
            TokenType::Bang => {
                if value_ty != ValueType::Bool {
                    let err_ty =
                        SemErrType::OpTypeMismatch(ValueType::Bool, Operator::Bang, value_ty);
                    return Err(SemErr::new(line, err_ty));
                }
                Ok(value_ty)
            }
            _ => Err(SemErr::new(line, SemErrType::InvalidPrefix)),
        }
    }

    fn check_if_params_and_args_correspond(
        &mut self,
        args: &mut [Expr<'a>],
        parameters: Vec<ValueType>,
        name: String,
        line: u32,
    ) -> Result<(), SemErr> {
        if args.len() != parameters.len() {
            let err_ty = SemErrType::IncorrectArity(
                name.to_string(),
                parameters.len() as u8,
                args.len() as u8,
            );
            return Err(SemErr::new(line, err_ty));
        }

        for (i, arg) in args.iter_mut().enumerate() {
            let arg_ty = self.analyse_expr(arg)?;

            let param_ty = &parameters[i];

            let is_exact_match = arg_ty == *param_ty;
            let is_any = *param_ty == ValueType::Any;
            let is_array_match = matches!(param_ty, ValueType::Arr(inner) if **inner == ValueType::Any)
                && matches!(arg_ty, ValueType::Arr(_));

            if !is_exact_match && !is_any && !is_array_match && !try_coerce(&mut arg.expr, param_ty)
            {
                let err_ty =
                    SemErrType::ParamTypeMismatch(name.to_string(), param_ty.clone(), arg_ty);
                return Err(SemErr::new(line, err_ty));
            }
        }
        Ok(())
    }

    fn analyse_dot(
        &mut self,
        new_value: Option<&mut Box<Expr<'a>>>,
        inst: &mut Box<Expr<'a>>,
        line: u32,
        property: &str,
    ) -> Result<(ValueType, ExprType<'a>), SemErr> {
        let name = if let ExprType::This = inst.expr {
            
            let Some(name) = self.current_struct else {
                let ty = SemErrType::SelfOutsideStruct;
                return Err(SemErr::new(line, ty));
            };

            if !self.current_use_self {
                let ty = SemErrType::SelfInMethodWithoutSelfParam;
                return Err(SemErr::new(line, ty));
            }
            name.to_string()
        } else {
            let inst_ty = self.analyse_expr(inst)?;
            let ValueType::Struct(name) = inst_ty else {
                let ty = SemErrType::InvalidTypeFieldAccess(inst_ty);
                return Err(SemErr::new(line, ty));
            };
            name
        };
        let Some(data) = self.enities.structs.get(&name as &str) else {
            let ty = SemErrType::UndefinedStruct(name);
            return Err(SemErr::new(line, ty));
        };

        let index = data.get_field_index(name, property, line)?;
        let field_ty = data.fields[index as usize].clone().0;

        let expr = if let Some(new_value) = new_value {
            let new_value_ty = self.analyse_expr(new_value)?;
            if new_value_ty != field_ty && !try_coerce(&mut new_value.expr, &field_ty) {
                let err_ty = SemErrType::FieldTypeMismatch(field_ty, new_value_ty);
                return Err(SemErr::new(line, err_ty));
            }
            ExprType::DotAssignResolved {
                inst: inst.clone(),
                new_value: new_value.clone(),
                index,
            }
        } else {
            ExprType::DotResolved {
                inst: inst.clone(),
                index,
            }
        };

        Ok((field_ty, expr))
    }

    fn get_called_func_data(
        &mut self,
        name: &'a str,
        line: u32,
    ) -> Result<(ValueType, Vec<ValueType>), SemErr> {
        if let Some(data) = self.enities.structs.get(name) {
            let params = data.fields.iter().map(|(ty, _)| ty.clone()).collect();
            let return_ty = ValueType::Struct(name.to_string());
            return Ok((return_ty, params));
        }

        if let Some(data) = self.enities.nat_structs.get(name) {
            let params = data.fields.iter().map(|(ty, _)| ty.clone()).collect();
            let return_ty = ValueType::Struct(name.to_string());
            return Ok((return_ty, params));
        }

        if let Some(data) = self.enities.funcs.get(name) {
            let parameters = data.parameters.iter().map(|p| p.0.clone()).collect();
            let return_ty = data.return_ty.clone();

            return Ok((return_ty, parameters));
        };

        let ty = SemErrType::UndefinedFunc(name.to_string());
        Err(SemErr::new(line, ty))
    }
}

fn try_coerce(expr: &mut ExprType, target: &ValueType) -> bool {
    match expr {
        ExprType::Lit(lit) => match (&lit, target) {
            (Literal::I64(n), ValueType::U64) => {
                *lit = Literal::U64(*n as u64);
                true
            }
            _ => false,
        },
        ExprType::Binary { left, right, .. } => {
            try_coerce(&mut left.expr, target) && try_coerce(&mut right.expr, target)
        }
        _ => false,
    }
}
