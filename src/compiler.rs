use std::{borrow::BorrowMut, collections::HashMap};

use crate::{
    error::{print_error, ParseError},
    expression::{Expr, ExprType},
    func_compiler::FuncCompilerStack,
    native_funcs,
    object::{Heap, ObjFunc, ObjNative, Object},
    op_code::OpCode,
    statement::{Stmt, StmtType},
    token::{Literal, TokenType},
    value::StackValue,
};

pub struct Compiler<'a> {
    heap: Heap,
    comps: FuncCompilerStack<'a>,
    funcs: HashMap<&'a str, StackValue>,
}
impl<'a> Compiler<'a> {
    fn new() -> Self {
        Self {
            heap: Heap::new(),
            comps: FuncCompilerStack::new(),
            funcs: HashMap::new(),
        }
    }
    pub fn compile(stmts: Vec<Stmt>) -> Option<(ObjFunc, Heap)> {
        let mut comp = Compiler::new();
        comp.collect_type_data(&stmts);

        for stmt in stmts {
            if let Err(err) = comp.emit_stmt(stmt) {
                print_error(err.line, &err.msg);

                return None;
            }
        }

        let func = comp.comps.end_compiler(69);
        Some((func, comp.heap))
    }

    fn collect_type_data(&mut self, stmts: &Vec<Stmt<'a>>) {
        macro_rules! add_func {
            ($name: expr, $func: ident) => {
                let func = ObjNative::new($name.to_string(), native_funcs::$func);
                let (func, _) = self.heap.alloc(func, Object::Native);
                let value = StackValue::Obj(func);
                self.funcs.insert($name, value);
            };
        }
        add_func!("clock", clock);
        add_func!("print", print);
        add_func!("println", println);
        add_func!("sin", sin);
        add_func!("cos", cos);
        add_func!("sin", tan);
        add_func!("min", min);
        add_func!("max", max);
        add_func!("abs", abs);
        add_func!("sqrt", sqrt);
        add_func!("pow", pow);

        for stmt in stmts {
            let line = stmt.line;
            if let StmtType::Func {
                name,
                parameters,
                body,
                return_ty,
            } = &stmt.stmt
            {
                let dummy = ObjFunc::new(name.to_string(), *return_ty);
                let (mut func_obj, _) = self.heap.alloc(dummy, Object::Func);
                self.funcs.insert(*name, StackValue::Obj(func_obj));

                self.comps.push(name.to_string(), *return_ty);
                self.comps.begin_scope();
                for (ty, name) in parameters {
                    self.comps.add_local(name, *ty, line).unwrap();
                }

                self.emit_stmt(*body.clone()).unwrap();

                self.comps.emit_return(line);

                let compiled_func = self.comps.end_compiler(line);
                if let Object::Func(ref mut func) = func_obj.borrow_mut() {
                    func.data = compiled_func;
                }
            }
        }
    }

    fn emit_stmt(&mut self, stmt: Stmt<'a>) -> Result<(), ParseError> {
        let line = stmt.line;
        match stmt.stmt {
            StmtType::Expr(expr) => {
                self.emit_expr(expr)?;
                self.comps.emit_byte(OpCode::Pop as u8, line);
            }
            StmtType::Println(expr) => {
                self.emit_expr(expr)?;
                self.comps.emit_byte(OpCode::Print as u8, line);
            }
            StmtType::Var { name, value, ty } => {
                self.emit_expr(value)?;
                self.comps.add_local(name, ty, line)?;
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
                self.emit_expr(condition)?;

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
                self.emit_expr(condition)?;

                let exit_jump = self.comps.emit_jump(OpCode::JumpIfFalse, line);
                self.comps.emit_byte(OpCode::Pop as u8, line);
                self.emit_stmt(*body)?;
                self.comps.emit_loop(loop_start, line)?;

                self.comps.patch_jump(exit_jump)?;
                self.comps.emit_byte(OpCode::Pop as u8, line);
            }
            StmtType::For {
                condition,
                body,
                var,
            } => {
                self.emit_stmt(*var)?;
                let var_arg = self.comps.get_local_count() as u8 - 1;
                let loop_start = self.comps.get_code_len();
                self.emit_expr(condition)?;

                let exit_jump = self.comps.emit_jump(OpCode::JumpIfFalse, line);
                self.comps.emit_byte(OpCode::Pop as u8, line);
                self.emit_stmt(*body)?;

                self.comps.emit_bytes(OpCode::GetLocal as u8, var_arg, line);
                self.comps.emit_constant(StackValue::F64(1.), line)?;

                self.comps.emit_byte(OpCode::Add as u8, line);
                self.comps.emit_bytes(OpCode::SetLocal as u8, var_arg, line);
                self.comps.emit_byte(OpCode::Pop as u8, line);

                self.comps.emit_loop(loop_start, line)?;

                self.comps.patch_jump(exit_jump)?;
                self.comps.emit_byte(OpCode::Pop as u8, line);

                // necessary so the variable goes out of scope again
                self.comps.emit_byte(OpCode::Pop as u8, line);
                self.comps.decrement_local_count();
            }
            StmtType::Func {
                name: _,
                parameters: _,
                body: _,
                return_ty: _,
            } => {}
            StmtType::Return(value) => {
                self.emit_expr(value)?;
                self.comps.emit_byte(OpCode::Return as u8, line);
            }
        }
        Ok(())
    }

    fn emit_expr(&mut self, expr: Expr) -> Result<(), ParseError> {
        let line = expr.line;
        match expr.expr {
            ExprType::Lit(lit) => match lit {
                Literal::None => unreachable!(),
                Literal::Str(str) => {
                    let (object, _) = self.heap.alloc(str.to_string(), Object::Str);
                    let stack_value = StackValue::Obj(object);
                    self.comps.emit_constant(stack_value, line)?;
                }
                Literal::Num(num) => self.comps.emit_constant(StackValue::F64(num), line)?,
                Literal::True => self.comps.emit_byte(OpCode::True as u8, line),
                Literal::False => self.comps.emit_byte(OpCode::False as u8, line),
                Literal::Null => self.comps.emit_byte(OpCode::Null as u8, line),
            },
            ExprType::Var(name) => {
                if let Some((arg, _kind)) = self.comps.resolve_local(name) {
                    self.comps.emit_bytes(OpCode::GetLocal as u8, arg, line);
                } else {
                    // TODO: report these errors in earlier stage
                    let msg = format!("The variable/function with name '{}' does not exist.", name);
                    return Err(ParseError::new(line, &msg));
                }
            }
            ExprType::Assign { name, value } => {
                if let Some((arg, _kind)) = self.comps.resolve_local(name) {
                    self.emit_expr(*value)?;
                    self.comps.emit_bytes(OpCode::SetLocal as u8, arg, line);
                } else {
                    let msg = format!("The variable with name '{}' does not exist.", name);
                    return Err(ParseError::new(line, &msg));
                }
            }
            ExprType::Unary {
                prefix,
                value: right,
            } => {
                self.emit_expr(*right)?;
                match prefix {
                    TokenType::Minus => self.comps.emit_byte(OpCode::Negate as u8, line),
                    TokenType::Bang => self.comps.emit_byte(OpCode::Not as u8, line),
                    _ => unreachable!(),
                }
            }
            ExprType::Binary { left, op, right } => {
                self.emit_expr(*left)?;
                self.emit_expr(*right)?;
                let op_code = op.to_op_code();
                self.comps.emit_byte(op_code as u8, line);
            }
            ExprType::Call { name, args } => {
                let fn_ptr = *self.funcs.get(name).unwrap();
                self.comps.emit_constant(fn_ptr, line)?;

                for var in args.clone() {
                    self.emit_expr(var)?;
                }
                self.comps
                    .emit_bytes(OpCode::Call as u8, args.len() as u8 + 1, line);
            }
        };
        Ok(())
    }
}
