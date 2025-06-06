use crate::{error::EmitErr, object::ObjFunc, op_code::OpCode, value::StackValue};

#[derive(Debug)]
pub struct FuncCompilerStack<'a> {
    comps: Vec<FuncCompiler<'a>>,
    current: usize,
}
impl<'a> FuncCompilerStack<'a> {
    pub fn new() -> Self {
        Self {
            comps: vec![],
            current: 0,
        }
    }

    pub fn begin_scope(&mut self) {
        self.comps[self.current].scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.comps[self.current].scope_depth -= 1;

        while self.should_remove_local() {
            self.emit_byte(OpCode::Pop as u8, 69);
            self.comps[self.current].local_count -= 1;
        }
    }

    pub fn end_compiler(&mut self, line: u32) -> ObjFunc {
        self.emit_return(line);

        self.current = 0;
        self.comps.pop().unwrap().get_func()
    }

    pub fn emit_return(&mut self, line: u32) {
        self.emit_byte(OpCode::Null as u8, line);
        self.emit_byte(OpCode::Return as u8, line);
    }

    pub fn emit_constant(&mut self, value: StackValue, line: u32) -> Result<(), EmitErr> {
        let const_index = self.make_constant(value, line)?;
        self.emit_bytes(OpCode::Constant as u8, const_index, line);
        Ok(())
    }

    fn make_constant(&mut self, value: StackValue, line: u32) -> Result<u8, EmitErr> {
        let const_index = self.add_constant(value);
        if const_index > u8::MAX.into() {
            let msg = "Too many constants in one chunk.";
            return Err(EmitErr::new(line, msg));
        }
        Ok(const_index as u8)
    }

    pub fn emit_byte(&mut self, byte: u8, line: u32) {
        self.write_byte_to_chunk(byte, line);
    }

    pub fn emit_bytes(&mut self, byte_0: u8, byte_1: u8, line: u32) {
        self.emit_byte(byte_0, line);
        self.emit_byte(byte_1, line);
    }

    pub fn emit_jump(&mut self, instruction: OpCode, line: u32) -> usize {
        self.emit_byte(instruction as u8, line);
        self.emit_byte(0xFF, line);
        self.emit_byte(0xFF, line);
        self.get_code_len() - 2
    }

    pub fn emit_loop(&mut self, loop_start: usize, line: u32) -> Result<(), EmitErr> {
        self.emit_byte(OpCode::Loop as u8, line);

        let offset = self.get_code_len() - loop_start + 2;
        if offset > u16::MAX as usize {
            let msg = "Loop body too large.";
            return Err(EmitErr::new(line, msg));
        }

        self.emit_byte(((offset >> 8) & 0xFF) as u8, line);
        self.emit_byte((offset & 0xFF) as u8, line);
        Ok(())
    }

    pub fn decrement_local_count(&mut self) {
        self.comps[self.current].local_count -= 1;
    }

    fn add_constant(&mut self, value: StackValue) -> usize {
        self.comps[self.current].func.chunk.add_constant(value)
    }

    fn write_byte_to_chunk(&mut self, byte: u8, line: u32) {
        self.comps[self.current]
            .func
            .chunk
            .write_byte_to_chunk(byte, line);
    }

    pub fn get_code_len(&self) -> usize {
        self.comps[self.current].func.chunk.code.len()
    }

    pub fn get_local_count(&self) -> usize {
        self.comps[self.current].local_count
    }

    pub fn add_local(&mut self, name: &'a str, line: u32) -> Result<(), EmitErr> {
        if self.current().local_count == MAX_LOCAL_AMT {
            return Err(EmitErr::new(line, "Too many locals."));
        }

        let local = Local::new(name, self.current().scope_depth);

        let local_count = self.current().local_count;
        self.comps[self.current].locals[local_count] = local;
        self.comps[self.current].local_count += 1;
        Ok(())
    }

    pub fn patch_jump(&mut self, offset: usize) -> Result<(), EmitErr> {
        let jump = self.get_code_len() - offset - 2;

        if jump > u16::MAX as usize {
            let msg = "Too much code to jump over.";
            return Err(EmitErr::new(0, msg));
        }

        self.comps[self.current].func.chunk.code[offset] = ((jump >> 8) & 0xFF) as u8;
        self.comps[self.current].func.chunk.code[offset + 1] = (jump & 0xFF) as u8;
        Ok(())
    }

    pub fn patch_jump_to(&mut self, from: usize, to: usize, line: u32) -> Result<(), EmitErr> {
        let jump = from
            .checked_sub(to - 2)
            .ok_or_else(|| EmitErr::new(line, "Invalid jump target."))?;

        if jump > u16::MAX as usize {
            let msg = "Too much code to jump over.";
            return Err(EmitErr::new(line, msg));
        }

        self.comps[self.current].func.chunk.code[from] = ((jump >> 8) & 0xFF) as u8;
        self.comps[self.current].func.chunk.code[from + 1] = (jump & 0xFF) as u8;
        Ok(())
    }

    pub fn push(&mut self, func_name: String) {
        let new_compiler = FuncCompiler::new(func_name);
        self.comps.push(new_compiler);
        self.current = self.comps.len() - 1;
    }

    pub fn add_continue(&mut self, line: u32) -> Result<(), EmitErr> {
        if self.current().continue_stack.is_empty() {
            let msg = "'continue' can only be used inside loops.";
            return Err(EmitErr::new(line, msg));
        }

        let jump = self.emit_jump(OpCode::Loop, line);
        self.comps[self.current]
            .continue_stack
            .last_mut()
            .unwrap()
            .push(jump);
        Ok(())
    }

    pub fn push_new_continue_stack(&mut self) {
        self.comps[self.current].continue_stack.push(vec![]);
    }

    pub fn patch_continues(&mut self, loop_start: usize, line: u32) -> Result<(), EmitErr> {
        let continues = self.comps[self.current].continue_stack.pop().unwrap();
        for from in continues {
            self.patch_jump_to(from, loop_start, line)?;
        }
        Ok(())
    }

    pub fn add_break(&mut self, line: u32) -> Result<(), EmitErr> {
        if self.current().break_stack.is_empty() {
            return Err(EmitErr::new(line, "'break' can only be used inside loops."));
        }

        let jump = self.emit_jump(OpCode::Jump, line);
        self.comps[self.current]
            .break_stack
            .last_mut()
            .unwrap()
            .push(jump);

        Ok(())
    }

    pub fn push_new_break_stack(&mut self) {
        self.comps[self.current].break_stack.push(vec![]);
    }

    pub fn patch_breaks(&mut self) -> Result<(), EmitErr> {
        let breaks = self.comps[self.current].break_stack.pop().unwrap();
        for jump in breaks {
            self.patch_jump(jump)?;
        }
        Ok(())
    }

    pub fn resolve_local(&mut self, name: &str) -> Option<u8> {
        for i in (0..self.current().local_count).rev() {
            if self.current().locals[i].name == name {
                return Some(i as u8);
            }
        }
        None
    }

    fn should_remove_local(&self) -> bool {
        let depth = self.current().locals[self.current().local_count - 1].depth;
        self.current().local_count > 0 && depth > self.current().scope_depth
    }

    fn current(&self) -> &FuncCompiler {
        &self.comps[self.current]
    }
}

#[derive(Debug, Clone, Copy)]
struct Local<'a> {
    name: &'a str,
    depth: usize,
}
impl<'a> Local<'a> {
    fn new(name: &'a str, depth: usize) -> Self {
        Self { name, depth }
    }
}

const MAX_LOCAL_AMT: usize = u8::MAX as usize;

#[derive(Debug)]
pub struct FuncCompiler<'a> {
    locals: [Local<'a>; MAX_LOCAL_AMT],
    local_count: usize,
    scope_depth: usize,
    func: ObjFunc,
    break_stack: Vec<Vec<usize>>,
    continue_stack: Vec<Vec<usize>>,
}
impl<'a> FuncCompiler<'a> {
    pub fn new(func_name: String) -> Self {
        let local = Local::new("", 0);
        Self {
            locals: [local; MAX_LOCAL_AMT],
            local_count: 1,
            scope_depth: 0,
            func: ObjFunc::new(func_name),
            break_stack: vec![],
            continue_stack: vec![],
        }
    }

    pub fn get_func(self) -> ObjFunc {
        self.func
    }
}
