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

    let a = chunk.add_constant(StackValue::F64(1.));
    let b = chunk.add_constant(StackValue::F64(2.));
    let c = chunk.add_constant(StackValue::F64(3.));

    chunk.write_byte_to_chunk(OpCode::Constant as u8, 123);
    chunk.write_byte_to_chunk(a as u8, 123);

    chunk.write_byte_to_chunk(OpCode::Constant as u8, 123);
    chunk.write_byte_to_chunk(b as u8, 123);

    chunk.write_byte_to_chunk(OpCode::Mul as u8, 123);

    chunk.write_byte_to_chunk(OpCode::Constant as u8, 123);
    chunk.write_byte_to_chunk(c as u8, 123);

    chunk.write_byte_to_chunk(OpCode::Add as u8, 123);

    // chunk.disassemble("test");

    // let x = vec![1, 2, 3];
    // x.le/
    chunk.write_byte_to_chunk(OpCode::Return as u8, 123);
    VM::interpret(chunk);
}
