use analysis::Analyser;
use emitter::Emitter;
use error::PRINT_TOKENS;
use op_code::OpCode;
use scanner::Scanner;
use value::StackValue;

use colored::Colorize;

mod analysis;
mod analysis_types;
mod chunk;
mod emitter;
mod error;
mod expression;
mod func_compiler;
mod heap;
mod native_funcs;
mod object;
mod op_code;
mod parse_types;
mod parser;
mod scanner;
mod statement;
mod token;
mod value;
mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let source = if args.len() <= 1 {
        let msg = "Could not find file.crs. The file should be in the same directory as either the executable file or Cargo.toml.";
        std::fs::read_to_string("file.crs").expect(msg)
    } else {
        let msg = format!("Could not find file '{}'.", args[1]);
        std::fs::read_to_string(&args[1]).expect(&msg)
    };

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

    if PRINT_TOKENS {
        for token in &tokens {
            println!("{:?} type: {:?}", token, token.ty as u8);
        }
        println!();
    }

    let mut statements = match parser::Parser::compile(tokens) {
        Some(statements) => statements,
        None => {
            println!(
                "{}",
                "Parse error(s) detected, terminating program.".purple()
            );
            return;
        }
    };
    // dbg!(&statements);

    let (func_data, nat_func_data, struct_data) = match Analyser::analyse_stmts(&mut statements) {
        Some(func_data) => func_data,
        None => return,
    };

    // dbg!(&statements);
    if let Some((func, heap)) = Emitter::compile(statements, func_data, nat_func_data, struct_data)
    {
        vm::VM::interpret(func, heap);
    }
}
