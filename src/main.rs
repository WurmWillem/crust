use chunk::Chunk;
use compiler::Compiler;
use error::PRINT_SCAN_TOKENS;
use opcode::OpCode;
use scanner::Scanner;
use value::StackValue;
use vm::VM;

use colored::Colorize;

mod chunk;
mod compiler;
mod compiler_types;
mod error;
mod object;
mod opcode;
mod scanner;
mod token;
mod value;
mod vm;

fn main() {
    let msg = "file.crust is niet gevonden. Het moet in dezelfde directory als de binary of Cargo.toml zitten.";
    let source = std::fs::read_to_string("file.js").expect(msg);

    let scanner = Scanner::new(&source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(_) => {
            println!("{}", "Scan error(s) detected, terminate program.".purple());
            return;
        }
    };
    
    if PRINT_SCAN_TOKENS {
        for token in &tokens {
            println!("{:?}", token);
        }
        println!();
    }

    let (chunk, objects) = match Compiler::compile(tokens, Chunk::new()) {
        None => {
            return;
        }
        Some((chunk, objects)) => (chunk, objects),
    };

    VM::interpret(chunk, objects);
}
