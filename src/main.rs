use vm::VM;

use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::value::StackValue;

mod chunk;
mod opcode;
mod value;
mod vm;

pub type Byte = u8;

fn main() {
    let mut chunk = Chunk::new();

    // let constant_index = chunk.add_constant(StackValue::F64(1.2));
    // chunk.write_byte_to_chunk(OpCode::Constant as u8, 123);
    // chunk.write_byte_to_chunk(constant_index as u8, 123);

    let constant_index = chunk.add_constant(StackValue::F64(3454.));
    chunk.write_byte_to_chunk(OpCode::Constant as u8, 123);
    chunk.write_byte_to_chunk(constant_index as u8, 123);
    chunk.write_byte_to_chunk(OpCode::Negate as u8, 123);

    chunk.write_byte_to_chunk(OpCode::Return as u8, 123);
    // chunk.disassemble("test");

    // let x = vec![1, 2, 3];
    // x.le/
    VM::interpret(chunk);
}

