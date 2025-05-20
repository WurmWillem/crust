use crate::{
    error::ParseError,
    func_compiler::FuncCompilerStack,
    object::{Heap, ObjFunc},
    opcode::OpCode,
    parser::{BinaryOp, Expr},
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
        Self { heap: Heap::new(), comps: FuncCompilerStack::new() }
    }
    pub fn compile(expr: Expr) -> Option<(ObjFunc, Heap)> {
        let mut comp = Comp::new();
        comp.emit_expr(expr).unwrap();
        comp.emit_byte(OpCode::Print as u8, 0);
        let func = comp.end_compiler(69);

        Some((func, comp.heap))
        // None
    }

    pub fn emit_expr(&mut self, expr: Expr) -> Result<(), ParseError> {
        match expr {
            Expr::Lit(lit, line) => match lit {
                Literal::None => unreachable!(),
                Literal::Str(_) => todo!(),
                Literal::Num(num) => {
                    self.emit_constant(StackValue::F64(num), line)?;
                }
                Literal::True => todo!(),
                Literal::False => todo!(),
                Literal::Null => todo!(),
            },
            Expr::Variable(_) => todo!(),
            Expr::Unary {
                prefix,
                value: right,
                line,
            } => {
                self.emit_expr(*right)?;
                match prefix {
                    TokenType::Minus => self.emit_byte(OpCode::Negate as u8, line),
                    TokenType::Bang => self.emit_byte(OpCode::Not as u8, line),
                    _ => unreachable!()
                }
            }            Expr::Binary {
                left,
                op,
                right,
                line,
            } => {
                self.emit_expr(*left)?;
                self.emit_expr(*right)?;
                match op {
                    BinaryOp::Add => self.emit_byte(OpCode::Add as u8, line),
                    BinaryOp::Sub => self.emit_byte(OpCode::Sub as u8, line),
                    BinaryOp::Mul => self.emit_byte(OpCode::Mul as u8, line),
                    BinaryOp::Div => self.emit_byte(OpCode::Div as u8, line),
                    _ => unreachable!()
                }
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
