use comp::Comp;
use error::PRINT_SCAN_TOKENS;
use op_code::OpCode;
use scanner::Scanner;
use value::StackValue;

use colored::Colorize;

mod chunk;
mod comp;
mod compiler_types;
mod error;
mod func_compiler;
mod native_funcs;
mod object;
mod op_code;
mod parse_types;
mod parser;
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

    let statements = match parser::Parser::compile(tokens) {
        Some(statements) => statements,
        None => {
            println!(
                "{}",
                "Compile error(s) detected, terminating program.".purple()
            );
            return;
        }
    };
    // dbg!(&statements);
    if let Some((func, heap)) = Comp::compile(statements) {
        vm::VM::interpret(func, heap);
    } else {
        return;
    }
}
