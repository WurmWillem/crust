use crate::{OpCode, StackValue};

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<StackValue>,
    lines: Vec<u32>,
}
impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn get_ptr(&self) -> *const u8 {
        self.code.as_ptr()
    }

    pub fn write_byte_to_chunk(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: StackValue) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&mut self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&mut self, offset: usize) -> usize {
        print!("{}  ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction.into() {
            OpCode::Return => Self::simple_instruction("OP_RETURN", offset),
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),

            OpCode::Null => Self::simple_instruction("OP_NULL", offset),
            OpCode::True => Self::simple_instruction("OP_TRUE", offset),
            OpCode::False => Self::simple_instruction("OP_FALSE", offset),

            OpCode::Negate => Self::simple_instruction("OP_NEGATE", offset),
            OpCode::Not => Self::simple_instruction("OP_NOT", offset),

            OpCode::Add => Self::simple_instruction("OP_ADD", offset),
            OpCode::Sub => Self::simple_instruction("OP_SUB", offset),
            OpCode::Mul => Self::simple_instruction("OP_MUL", offset),
            OpCode::Div => Self::simple_instruction("OP_DIV", offset),
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant_index = self.code[offset + 1];
        print!("{}  {}:", name, constant_index);
        println!(" '{}'", self.constants[constant_index as usize]);
        offset + 2
    }
}
