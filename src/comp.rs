use crate::{
    error::ParseError,
    func_compiler::FuncCompilerStack,
    object::{Heap, ObjFunc},
    opcode::OpCode,
    parser::Expr,
    token::Literal,
    value::StackValue,
};

pub struct Comp<'a> {
    heap: Heap,
    comps: FuncCompilerStack<'a>,
    // decl_types: DeclaredTypes<'token>,
}
impl<'a> Comp<'a> {
    pub fn new() -> Self {
        Self { heap: Heap::new(), comps: FuncCompilerStack::new() }
    }
    pub fn compile(&mut self, expr: Expr) -> Result<ObjFunc, ParseError> {
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
                right,
                line,
            } => todo!(),
            Expr::Binary {
                left,
                op,
                right,
                line,
            } => todo!(),
        };
        let func = self.end_compiler(69);
        Ok(func)
        // None
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
