pub type Byte = u8;

#[repr(u8)]
enum OpCode {
    Return,
}
impl std::convert::From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            _ => panic!("Not a valid opcode"),
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

struct Chunk {
    code: Vec<Byte>,
}
impl Chunk {
    fn new() -> Self {
        Self { code: vec![] }
    }

    fn write_byte_to_chunk(&mut self, byte: Byte) {
        self.code.push(byte);
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

        let instruction = self.code[offset];
        match instruction.into() {
            OpCode::Return => simple_instruction("OP_RETURN", offset),
            _ => panic!("Unreachable."),
        }
    }
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_byte_to_chunk(OpCode::Return as u8);
    chunk.disassemble("test");
    // let x = vec![1, 2, 3];
    // x.le/
}
