use chunk::Chunk;
use colored::Colorize;
use parser::Parser;
use scanner::Scanner;
use vm::VM;

use crate::opcode::OpCode;
use crate::value::StackValue;

mod chunk;
mod error;
mod opcode;
mod parser;
mod scanner;
mod token;
mod value;
mod vm;

const PRINT_SCAN_TOKENS: bool = false;

fn main() {
    let msg = "file.crust is niet gevonden. Het moet in dezelfde directory als de binary of Cargo.toml zitten.";
    let source = std::fs::read_to_string("file.crust").expect(msg);

    let scanner = Scanner::new(&source);
    // let tokens = scanner.scan_tokens().unwrap();
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

    let chunk = Parser::compile(tokens, Chunk::new());
    VM::interpret(chunk);
}
