use crate::{
    error::ParseError,
    object::ObjFunc,
    value::{StackValue, ValueType},
};

pub struct FuncCompilerStack<'a> {
    comps: Vec<FuncCompiler<'a>>,
    current: usize,
}
impl<'a> FuncCompilerStack<'a> {
    pub fn new() -> Self {
        let root = FuncCompiler::new("".to_string(), ValueType::Null);
        Self {
            comps: vec![root],
            current: 0,
        }
    }

    pub fn get_return_type(&self) -> ValueType {
        self.current().func.return_type
    }

    pub fn increment_scope_depth(&mut self) {
        self.comps[self.current].scope_depth += 1;
    }

    pub fn decrement_scope_depth(&mut self) {
        self.comps[self.current].scope_depth -= 1;
    }

    pub fn decrement_local_count(&mut self) {
        self.comps[self.current].local_count -= 1;
    }

    pub fn add_constant(&mut self, value: StackValue) -> usize {
        self.comps[self.current].func.chunk.add_constant(value)
    }

    pub fn write_byte_to_chunk(&mut self, byte: u8, line: u32) {
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

    pub fn add_local(
        &mut self,
        name: &'a str,
        kind: ValueType,
        line: u32,
    ) -> Result<(), ParseError> {
        if self.current().local_count == MAX_LOCAL_AMT {
            let msg = "Too many local variables in function.";
            return Err(ParseError::new(line, msg));
        }

        let local = Local::new(name, self.current().scope_depth, kind);

        let local_count = self.current().local_count;
        self.comps[self.current].locals[local_count] = local;
        self.comps[self.current].local_count += 1;
        Ok(())
    }

    pub fn patch_jump(&mut self, offset: usize) -> Result<(), ParseError> {
        let jump = self.get_code_len() - offset - 2;

        if jump > u16::MAX as usize {
            let msg = "Too much code to jump over.";
            return Err(ParseError::new(0, msg));
        }

        self.comps[self.current].func.chunk.code[offset] = ((jump >> 8) & 0xFF) as u8;
        self.comps[self.current].func.chunk.code[offset + 1] = (jump & 0xFF) as u8;
        Ok(())
    }

    pub fn patch_return_type(&mut self, return_type: ValueType) {
        self.comps[self.current].patch_return_type(return_type);
    }

    pub fn push(&mut self, func_name: String, return_ty: ValueType) {
        let new_compiler = FuncCompiler::new(func_name, return_ty);
        self.comps.push(new_compiler);
        self.current = self.comps.len() - 1;
    }

    pub fn pop(&mut self) -> FuncCompiler {
        self.current = 0;
        self.comps.pop().unwrap()
    }

    pub fn resolve_local(&mut self, name: &str) -> Option<(u8, ValueType)> {
        // TODO: shadowing doesn't remove the old var as of now
        for i in (0..self.current().local_count).rev() {
            if self.current().locals[i].name == name {
                return Some((i as u8, self.current().locals[i].ty));
            }
        }
        None
    }

    pub fn should_remove_local(&self) -> bool {
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
    ty: ValueType,
    depth: usize,
}
impl<'a> Local<'a> {
    fn new(name: &'a str, depth: usize, ty: ValueType) -> Self {
        Self { name, depth, ty }
    }
}

const MAX_LOCAL_AMT: usize = u8::MAX as usize;
pub struct FuncCompiler<'a> {
    locals: [Local<'a>; MAX_LOCAL_AMT],
    local_count: usize,
    scope_depth: usize,
    func: ObjFunc,
}
impl<'a> FuncCompiler<'a> {
    pub fn new(func_name: String, return_ty: ValueType) -> Self {
        let local = Local::new("", 0, ValueType::None);
        Self {
            locals: [local; MAX_LOCAL_AMT],
            local_count: 1,
            scope_depth: 0,
            func: ObjFunc::new(func_name, return_ty),
        }
    }

    pub fn get_func(self) -> ObjFunc {
        self.func
    }

    pub fn patch_return_type(&mut self, return_type: ValueType) {
        self.func.return_type = return_type;
    }
}
