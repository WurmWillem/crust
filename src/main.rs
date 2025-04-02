use std::fmt::Display;

pub type Byte = u8;

#[repr(u8)]
enum OpCode {
    Return,
    Constant,
}
impl std::convert::From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            _ => panic!("Not a valid opcode"),
        }
    }
}

#[derive(Debug)]
enum Value {
    F64(f64),
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::F64(value) => write!(f, "{:?}", value),
        }
    }
}
// enum ValueType {
//     Bool,
//     Num,
//     Nil,
// }

struct Chunk {
    code: Vec<Byte>,
    constants: Vec<Value>,
    lines: Vec<u32>,
}
impl Chunk {
    fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    fn write_byte_to_chunk(&mut self, byte: Byte, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn disassemble(&mut self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&mut self, offset: usize) -> usize {
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
            _ => panic!("Unreachable."),
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

fn main() {
    let mut chunk = Chunk::new();

    let constant_index = chunk.add_constant(Value::F64(1.2));
    chunk.write_byte_to_chunk(OpCode::Constant as u8, 123);
    chunk.write_byte_to_chunk(constant_index as u8, 123);

    chunk.write_byte_to_chunk(OpCode::Return as u8, 123);
    chunk.disassemble("test");
    // let x = vec![1, 2, 3];
    // x.le/
}
