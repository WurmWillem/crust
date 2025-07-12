use std::{borrow::BorrowMut, collections::HashMap};

use crate::{
    analysis_types::{EnityData, FuncData},
    error::{print_error, EmitErr},
    expression::{Expr, ExprType},
    func_compiler::FuncCompilerStack,
    heap::Heap,
    object::{ObjFunc, ObjNative, Object},
    op_code::OpCode,
    statement::{Stmt, StmtType},
    token::{Literal, TokenType},
    value::{StackValue, ValueType},
};

pub struct Emitter<'a> {
    heap: Heap,
    comps: FuncCompilerStack<'a>,
    funcs: HashMap<&'a str, Vec<StackValue>>,
    structs: HashMap<&'a str, Vec<(&'a str, StackValue)>>,
}
impl<'a> Emitter<'a> {
    fn new() -> Self {
        Self {
            heap: Heap::new(),
            comps: FuncCompilerStack::new(),
            funcs: HashMap::new(),
            structs: HashMap::new(),
        }
    }
    pub fn compile(stmts: Vec<Stmt>, entities: EnityData) -> Option<(ObjFunc, Heap)> {
        let mut comp = Emitter::new();
        let func = match comp.init_funcs(entities) {
            Ok(func) => func,
            Err(err) => {
                print_error(err.line, &err.msg);
                return None;
            }
        };

        for stmt in stmts {
            if let Err(err) = comp.emit_stmt(stmt) {
                print_error(err.line, &err.msg);
                return None;
            }
        }

        Some((func, comp.heap))
    }

    fn init_funcs(&mut self, mut entities: EnityData<'a>) -> Result<ObjFunc, EmitErr> {
        for (name, data) in entities.nat_funcs.drain() {
            let mut values = vec![];
            for data in data {
                let func = ObjNative::new(name.to_string(), data.func);
                let (func, _) = self.heap.alloc_permanent(func, Object::Native);
                let value = StackValue::Obj(func);
                values.push(value);
            }
            self.funcs.insert(name, values);
        }

        // insert dummy function objects for recursion
        let mut func_objs = Vec::new();
        let func_data: Vec<(&'a str, FuncData<'a>)> = entities.funcs.drain().collect();
        for (name, _) in func_data.iter() {
            let dummy = ObjFunc::new(name.to_string());
            let (func_obj, _) = self.heap.alloc_permanent(dummy, Object::Func);

            self.funcs.insert(name, vec![StackValue::Obj(func_obj)]);
            func_objs.push(func_obj);
        }

        let mut method_objs = Vec::new();
        for (struct_name, data) in &entities.structs {
            let mut methods = vec![];
            for (name, _) in &data.methods {
                let dummy = ObjFunc::new(name.to_string());
                let (func_obj, _) = self.heap.alloc_permanent(dummy, Object::Func);

                methods.push((*name, StackValue::Obj(func_obj)));
                method_objs.push(func_obj);
            }
            self.structs.insert(struct_name, methods);
        }

        for (struct_name, data) in entities.nat_structs {
            let mut methods = vec![];
            for (name, data) in &data.methods {
                let func = ObjNative::new(name.to_string(), data.func);
                let (func, _) = self.heap.alloc_permanent(func, Object::Native);
                let value = StackValue::Obj(func);
                methods.push((*name, value));
            }
            self.structs.insert(struct_name, methods);
        }

        let mut main_func_obj = None;
        for (i, (name, data)) in func_data.into_iter().enumerate() {
            let line = data.line;

            self.comps.push(name.to_string());
            self.comps.begin_scope();
            for (_, name) in data.parameters {
                self.comps.add_local(name, line)?;
            }

            for stmt in data.body {
                self.emit_stmt(stmt)?;
            }

            self.comps.emit_return(line);

            let compiled_func = self.comps.end_compiler(line);
            if let Object::Func(ref mut func) = func_objs[i].borrow_mut() {
                if name == "main" {
                    main_func_obj = Some(compiled_func);
                } else {
                    func.data = compiled_func;
                }
            } else {
                unreachable!()
            }
        }

        for (_, data) in entities.structs {
            for (i, (name, data)) in data.methods.into_iter().enumerate() {
                let line = data.line;

                self.comps.push(name.to_string());
                self.comps.begin_scope();
                for (_, name) in data.parameters {
                    self.comps.add_local(name, line)?;
                }

                for stmt in data.body {
                    self.emit_stmt(stmt)?;
                }

                self.comps.emit_return(line);

                let compiled_func = self.comps.end_compiler(line);
                if let Object::Func(ref mut func) = method_objs[i].borrow_mut() {
                    func.data = compiled_func;
                } else {
                    unreachable!()
                }
            }
        }
        Ok(main_func_obj.unwrap())
    }

    fn emit_stmt(&mut self, stmt: Stmt<'a>) -> Result<(), EmitErr> {
        // dbg!(&stmt);
        let line = stmt.line;
        match stmt.stmt {
            StmtType::Expr(expr) => {
                self.emit_expr(&expr)?;
                self.comps.emit_byte(OpCode::Pop as u8, line);
            }
            StmtType::Println(expr) => {
                self.emit_expr(&expr)?;
                self.comps.emit_byte(OpCode::Print as u8, line);
            }
            StmtType::Var { name, value, ty: _ } => {
                self.comps.add_local(name, line)?;
                self.emit_expr(&value)?;
            }
            StmtType::Block(stmts) => {
                self.comps.begin_scope();
                for stmt in stmts {
                    self.emit_stmt(stmt)?;
                }
                self.comps.end_scope();
            }
            StmtType::If {
                final_else,
                condition,
                body,
            } => {
                self.emit_expr(&condition)?;

                let if_false_jump = self.comps.emit_jump(OpCode::JumpIfFalse, line);

                self.comps.emit_byte(OpCode::Pop as u8, line);
                self.emit_stmt(*body)?;

                let if_true_jump = self.comps.emit_jump(OpCode::Jump, line);

                self.comps.patch_jump(if_false_jump)?;
                self.comps.emit_byte(OpCode::Pop as u8, line);

                if let Some(final_else) = final_else {
                    self.emit_stmt(*final_else)?;
                }

                self.comps.patch_jump(if_true_jump)?;
            }
            StmtType::While { condition, body } => {
                let loop_start = self.comps.get_code_len();
                self.emit_expr(&condition)?;

                let exit_jump = self.comps.emit_jump(OpCode::JumpIfFalse, line);
                self.comps.emit_byte(OpCode::Pop as u8, line);

                self.comps.push_new_break_stack();
                self.comps.push_new_continue_stack();

                self.emit_stmt(*body)?;

                self.comps.patch_continues(loop_start, line)?;
                self.comps.emit_loop(loop_start, line)?;

                self.comps.patch_jump(exit_jump)?;
                self.comps.emit_byte(OpCode::Pop as u8, line);

                self.comps.patch_breaks()?;
            }
            StmtType::For {
                condition,
                body,
                var,
            } => {
                self.emit_stmt(*var)?;
                let var_arg = self.comps.get_local_count() as u8 - 1;
                let loop_start = self.comps.get_code_len();
                self.emit_expr(&condition)?;

                let exit_jump = self.comps.emit_jump(OpCode::JumpIfFalse, line);
                self.comps.emit_byte(OpCode::Pop as u8, line);

                self.comps.push_new_break_stack();
                self.emit_stmt(*body)?;

                self.comps.emit_bytes(OpCode::GetLocal as u8, var_arg, line);
                self.comps.emit_constant(StackValue::I64(1), line)?;

                self.comps.emit_byte(OpCode::Add as u8, line);
                self.comps.emit_bytes(OpCode::SetLocal as u8, var_arg, line);
                self.comps.emit_byte(OpCode::Pop as u8, line);

                self.comps.emit_loop(loop_start, line)?;

                self.comps.patch_jump(exit_jump)?;
                self.comps.emit_byte(OpCode::Pop as u8, line);

                self.comps.patch_breaks()?;

                // necessary so the variable goes out of scope again
                self.comps.emit_byte(OpCode::Pop as u8, line);
                self.comps.decrement_local_count();
            }
            StmtType::Func { .. } => {}
            StmtType::Return(value) => {
                self.emit_expr(&value)?;
                self.comps.emit_byte(OpCode::Return as u8, line);
            }
            StmtType::Break => {
                self.comps.add_break(line)?;
            }
            StmtType::Continue => {
                self.comps.add_continue(line)?;
            }
            StmtType::Struct { .. } => (),
            StmtType::Enum { .. } => (),
        }
        Ok(())
    }

    fn emit_expr(&mut self, expr: &Expr<'a>) -> Result<(), EmitErr> {
        let line = expr.line;
        match &expr.expr {
            ExprType::FuncCall { name, args, index } => {
                if let Some(methods) = self.structs.get(name) {
                    let method_len = methods.len() as u8;
                    for (_, value) in methods.iter().rev() {
                        self.comps.emit_constant(*value, line)?;
                    }
                    for var in args.iter().rev() {
                        self.emit_expr(var)?;
                    }

                    self.comps
                        .emit_bytes(OpCode::AllocInstance as u8, method_len, line);
                    self.comps.emit_byte(args.len() as u8, line);
                } else {
                    let fn_ptr = self.funcs.get(name).unwrap();
                    self.comps.emit_constant(fn_ptr[index.unwrap()], line)?;

                    for var in args {
                        self.emit_expr(var)?;
                    }

                    self.comps
                        .emit_bytes(OpCode::FuncCall as u8, args.len() as u8 + 1, line);
                }
            }
            ExprType::Array(arr) => {
                let arr_len = arr.len() as u64;
                for value in arr.iter().rev() {
                    self.emit_expr(value)?;
                }
                self.comps.emit_constant(StackValue::U64(arr_len), line)?;
                self.comps.emit_byte(OpCode::AllocArr as u8, line);
            }
            ExprType::Index { arr, index } => {
                self.emit_expr(arr)?;
                self.emit_expr(index)?;
                self.comps.emit_byte(OpCode::IndexArr as u8, line);
            }
            ExprType::AssignIndex {
                arr,
                index,
                new_value,
            } => {
                self.emit_expr(arr)?;
                self.emit_expr(index)?;
                self.emit_expr(new_value)?;
                self.comps.emit_byte(OpCode::AssignIndex as u8, line);
            }
            ExprType::DotResolved { inst, index } => {
                if let ExprType::This = inst.expr {
                    self.comps
                        .emit_bytes(OpCode::GetSelfField as u8, *index, line);
                } else {
                    self.emit_expr(inst)?;
                    self.comps
                        .emit_bytes(OpCode::GetPubField as u8, *index, line);
                }
            }
            ExprType::DotAssignResolved {
                inst,
                index,
                new_value,
            } => {
                if let ExprType::This = inst.expr {
                    self.emit_expr(new_value)?;
                    self.comps
                        .emit_bytes(OpCode::SetSelfField as u8, *index, line);
                } else {
                    self.emit_expr(inst)?;
                    self.emit_expr(new_value)?;
                    self.comps
                        .emit_bytes(OpCode::SetPubField as u8, *index, line);
                }
            }

            ExprType::MethodCallResolved {
                inst,
                index,
                args,
                use_self,
            } => {
                if !*use_self {
                    let ExprType::Identifier(name) = inst.expr else {
                        unreachable!()
                    };
                    let methods = self.structs.get(name).unwrap();
                    self.comps
                        .emit_constant(methods[(*index) as usize].1, line)?;

                    let args_len = args.len() as u8 + 1;
                    for var in args {
                        self.emit_expr(var)?;
                    }

                    self.comps
                        .emit_bytes(OpCode::FuncCall as u8, args_len, line);
                } else {
                    self.emit_expr(inst)?;
                    self.comps
                        .emit_bytes(OpCode::PushMethod as u8, *index, line);

                    let args_len = args.len() as u8 + 2;
                    for var in args {
                        self.emit_expr(var)?;
                    }

                    self.comps
                        .emit_bytes(OpCode::FuncCall as u8, args_len, line);
                }

                //self.comps.emit_byte(OpCode::Pop as u8, line);
            }
            ExprType::Lit(lit) => match lit {
                Literal::None => unreachable!(),
                Literal::Str(str) => {
                    let (object, _) = self.heap.alloc_permanent(str.to_string(), Object::Str);
                    let stack_value = StackValue::Obj(object);
                    self.comps.emit_constant(stack_value, line)?;
                }
                Literal::F64(num) => self.comps.emit_constant(StackValue::F64(*num), line)?,
                Literal::U64(num) => self.comps.emit_constant(StackValue::U64(*num), line)?,
                Literal::I64(num) => self.comps.emit_constant(StackValue::I64(*num), line)?,

                Literal::True => self.comps.emit_byte(OpCode::True as u8, line),
                Literal::False => self.comps.emit_byte(OpCode::False as u8, line),
                Literal::Null => self.comps.emit_byte(OpCode::Null as u8, line),
            },
            ExprType::Identifier(name) => {
                if let Some(arg) = self.comps.resolve_local(name) {
                    self.comps.emit_bytes(OpCode::GetLocal as u8, arg, line);
                } else if let Some(methods) = self.structs.get(name) {
                    self.comps.emit_constant(methods[0].1, line)?;

                    self.comps.emit_bytes(OpCode::FuncCall as u8, 1, line);
                    dbg!("struct");
                } else {
                    unreachable!()
                }
            }
            ExprType::Assign { name, new_value } => {
                let Some(arg) = self.comps.resolve_local(name) else {
                    unreachable!()
                };
                self.emit_expr(new_value)?;
                self.comps.emit_bytes(OpCode::SetLocal as u8, arg, line);
            }
            ExprType::Unary {
                prefix,
                value: right,
            } => {
                self.emit_expr(right)?;
                match prefix {
                    TokenType::Minus => self.comps.emit_byte(OpCode::Negate as u8, line),
                    TokenType::Bang => self.comps.emit_byte(OpCode::Not as u8, line),
                    _ => unreachable!(),
                }
            }
            ExprType::Binary { left, op, right } => {
                self.emit_expr(left)?;
                self.emit_expr(right)?;
                let op_code = op.to_op_code();
                self.comps.emit_byte(op_code as u8, line);
            }
            ExprType::Cast { value, target } => {
                self.emit_expr(value)?;
                match target {
                    ValueType::F64 => self.comps.emit_byte(OpCode::CastToF64 as u8, line),
                    ValueType::I64 => self.comps.emit_byte(OpCode::CastToI64 as u8, line),
                    ValueType::U64 => self.comps.emit_byte(OpCode::CastToU64 as u8, line),
                    _ => unreachable!(),
                }
            }
            ExprType::Dot { .. } => unreachable!(),
            ExprType::DotAssign { .. } => unreachable!(),
            ExprType::MethodCall { .. } => unreachable!(),
            ExprType::This => unreachable!(),
            ExprType::Colon { .. } => unreachable!(),
        };
        Ok(())
    }
}
