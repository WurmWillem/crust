use compiler::Parser;
use error::PRINT_SCAN_TOKENS;
use opcode::OpCode;
use scanner::Scanner;
use value::StackValue;
use vm::VM;

use colored::Colorize;

mod chunk;
mod compiler;
mod compiler_types;
mod declared_func;
mod error;
mod func_compiler;
mod native_funcs;
mod object;
mod opcode;
mod scanner;
mod token;
mod value;
mod vm;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let msg = "Could not find file.crust. The file should be in the same directory as either the executable file or Cargo.toml.";
    let source = std::fs::read_to_string("file.crs").expect(msg);

    let scanner = Scanner::new(&source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(_) => {
            println!(
                "{}",
                "Scan error(s) detected, terminating program.".purple()
            );
            return;
        }
    };

    if PRINT_SCAN_TOKENS {
        for token in &tokens {
            println!("{:?} type: {:?}", token, token.kind as u8);
        }
        println!();
    }

    let (func, heap, funcs) = match Parser::compile(tokens) {
        None => {
            return;
        }
        Some((func, heap, funcs)) => (func, heap, funcs),
    };

    VM::interpret(func, heap, funcs);
}
