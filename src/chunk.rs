use crate::{OpCode, StackValue};

#[derive(Debug)]
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

    // pub fn disassemble(&mut self, name: &str) {
    //     println!("== {} ==", name);
    //
    //     let mut offset = 0;
    //     while offset < self.code.len() {
    //         offset = self.disassemble_instruction(offset);
    //     }
    // }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("\n{} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        // dbg!(instruction);
        match instruction.into() {
            OpCode::Return => Self::simple_instruction("OP_RETURN", offset),
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::Pop => Self::simple_instruction("OP_POP", offset),
            // TODO: update for jump and vars and loop and call
            OpCode::Jump => Self::simple_instruction("OP_POP", offset),
            OpCode::JumpIfFalse => Self::simple_instruction("OP_POP", offset),
            OpCode::Loop => Self::simple_instruction("OP_POP", offset),

            OpCode::AllocInstance => Self::simple_instruction("OP_ALLOC_INSTANCE", offset),
            OpCode::GetPubField => Self::simple_instruction("OP_GET_PROPERTY", offset),
            OpCode::SetPubField => Self::simple_instruction("OP_SET_PROPERTY", offset),

            OpCode::AllocArr => Self::simple_instruction("OP_ALLOC_ARRAY", offset),
            OpCode::IndexArr => Self::simple_instruction("OP_INDEX_ARRAY", offset),
            OpCode::AssignIndex => Self::simple_instruction("OP_ASSIGN_INDEX", offset),

            OpCode::Print => Self::simple_instruction("OP_PRINT", offset),

            OpCode::FuncCall => Self::simple_instruction("OP_CALL", offset),
            OpCode::PushMethod => Self::simple_instruction("OP_METHOD_CALL", offset),

            OpCode::GetLocal => self.constant_instruction("OP_GET_LOCAL", offset),
            OpCode::SetLocal => self.constant_instruction("OP_SET_LOCAL", offset),
            OpCode::GetSelfField => self.constant_instruction("OP_GET_FIELD", offset),
            OpCode::GetSetField => self.constant_instruction("OP_SET_FIELD", offset),

            OpCode::Null => Self::simple_instruction("OP_NULL", offset),
            OpCode::True => Self::simple_instruction("OP_TRUE", offset),
            OpCode::False => Self::simple_instruction("OP_FALSE", offset),

            OpCode::Negate => Self::simple_instruction("OP_NEGATE", offset),
            OpCode::Not => Self::simple_instruction("OP_NOT", offset),

            OpCode::Add => Self::simple_instruction("OP_ADD", offset),
            OpCode::Sub => Self::simple_instruction("OP_SUB", offset),
            OpCode::Mul => Self::simple_instruction("OP_MUL", offset),
            OpCode::Div => Self::simple_instruction("OP_DIV", offset),

            OpCode::And => Self::simple_instruction("OP_AND", offset),
            OpCode::Or => Self::simple_instruction("OP_OR", offset),

            OpCode::Equal => Self::simple_instruction("OP_EQUAL", offset),
            OpCode::NotEqual => Self::simple_instruction("OP_BANG_EQUAL", offset),
            OpCode::Greater => Self::simple_instruction("OP_GREATER", offset),
            OpCode::GreaterEqual => Self::simple_instruction("OP_GREATER_EQUAL", offset),
            OpCode::Less => Self::simple_instruction("OP_LESS", offset),
            OpCode::LessEqual => Self::simple_instruction("OP_LESS_EQUAL", offset),
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant_index = self.code[offset + 1];
        print!("{}  {}:", name, constant_index);
        // println!(" '{}'", self.constants[constant_index as usize].display());
        // println!();
        offset + 2
    }
}
