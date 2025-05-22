use crate::{
    error::ParseError,
    func_compiler::FuncCompilerStack,
    object::{Heap, ObjFunc, Object},
    opcode::OpCode,
    parse_types::{Expr, ExprType, Stmt, StmtKind},
    token::{Literal, TokenType},
    value::StackValue,
};

pub struct Comp<'a> {
    heap: Heap,
    comps: FuncCompilerStack<'a>,
    // decl_types: DeclaredTypes<'token>,
}
impl<'a> Comp<'a> {
    fn new() -> Self {
        Self {
            heap: Heap::new(),
            comps: FuncCompilerStack::new(),
        }
    }
    pub fn compile(stmts: Vec<Stmt>) -> Option<(ObjFunc, Heap)> {
        let mut comp = Comp::new();

        for stmt in stmts {
            comp.emit_stmt(stmt).unwrap();
        }

        let func = comp.end_compiler(69);

        Some((func, comp.heap))
        // None
    }

    pub fn emit_stmt(&mut self, stmt: Stmt<'a>) -> Result<(), ParseError> {
        let line = stmt.line;
        match stmt.stmt {
            StmtKind::Expr(expr) => self.emit_expr(expr),
            StmtKind::Println(expr) => {
                // expr.lin
                self.emit_expr(expr)?;
                self.emit_byte(OpCode::Print as u8, line);
                Ok(())
            }
            StmtKind::Var { name, value, ty } => {
                self.emit_expr(value)?;
                self.comps.add_local(name, ty, line)?;
                Ok(())
            }
        }
    }
    pub fn emit_expr(&mut self, expr: Expr) -> Result<(), ParseError> {
        let line = expr.line;
        match expr.expr {
            ExprType::Lit(lit) => match lit {
                Literal::None => unreachable!(),
                Literal::Str(str) => {
                    let (object, _) = self.heap.alloc(str.to_string(), Object::Str);
                    let stack_value = StackValue::Obj(object);
                    self.emit_constant(stack_value, line)?;
                }
                Literal::Num(num) => self.emit_constant(StackValue::F64(num), line)?,
                Literal::True => self.emit_byte(OpCode::True as u8, line),
                Literal::False => self.emit_byte(OpCode::False as u8, line),
                Literal::Null => self.emit_byte(OpCode::Null as u8, line),
            },
            ExprType::Var(name) => {
                if let Some((arg, kind)) = self.comps.resolve_local(name) {
                    self.emit_bytes(OpCode::GetLocal as u8, arg, line);
                } else {
                    unreachable!()
                }
            }
            ExprType::Unary {
                prefix,
                value: right,
            } => {
                self.emit_expr(*right)?;
                match prefix {
                    TokenType::Minus => self.emit_byte(OpCode::Negate as u8, line),
                    TokenType::Bang => self.emit_byte(OpCode::Not as u8, line),
                    _ => unreachable!(),
                }
            }
            ExprType::Binary { left, op, right } => {
                self.emit_expr(*left)?;
                self.emit_expr(*right)?;
                let op_code = op.to_op_code();
                self.emit_byte(op_code as u8, line);
            }
        };
        Ok(())
    }

    fn end_compiler(&mut self, line: u32) -> ObjFunc {
        self.emit_return(line);
        self.comps.pop().get_func()
    }

    fn emit_return(&mut self, line: u32) {
        self.emit_byte(OpCode::Null as u8, line);
        self.emit_byte(OpCode::Return as u8, line);
    }

    fn emit_constant(&mut self, value: StackValue, line: u32) -> Result<(), ParseError> {
        let const_index = self.make_constant(value, line)?;
        self.emit_bytes(OpCode::Constant as u8, const_index, line);
        Ok(())
    }

    fn make_constant(&mut self, value: StackValue, line: u32) -> Result<u8, ParseError> {
        let const_index = self.comps.add_constant(value);
        if const_index > u16::MAX.into() {
            let msg = "Too many constants in one chunk.";
            return Err(ParseError::new(line, msg));
        }
        Ok(const_index as u8)
    }

    fn emit_byte(&mut self, byte: u8, line: u32) {
        self.comps.write_byte_to_chunk(byte, line);
    }

    fn emit_bytes(&mut self, byte_0: u8, byte_1: u8, line: u32) {
        self.emit_byte(byte_0, line);
        self.emit_byte(byte_1, line);
    }
}
