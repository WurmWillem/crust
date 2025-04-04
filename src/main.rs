use scanner::Scanner;

use crate::opcode::OpCode;
use crate::value::StackValue;

mod chunk;
mod opcode;
mod scanner;
mod value;
mod vm;
mod token;

pub type Byte = u8;

fn main() {
    let msg = "file.crust is niet gevonden. Het moet in dezelfde directory als de binary of Cargo.toml zitten.";
    let source = std::fs::read_to_string("file.crust").expect(msg);
    
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
}
