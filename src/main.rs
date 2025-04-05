use colored::Colorize;
use scanner::Scanner;

use crate::opcode::OpCode;
use crate::value::StackValue;

mod chunk;
mod opcode;
mod scanner;
mod token;
mod value;
mod vm;
mod parser;
mod error;

pub type Byte = u8;

fn main() {
    let msg = "file.crust is niet gevonden. Het moet in dezelfde directory als de binary of Cargo.toml zitten.";
    let source = std::fs::read_to_string("file.crust").expect(msg);

    let scanner = Scanner::new(&source);
    // let tokens = scanner.scan_tokens().unwrap();
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(_) => {
            println!(
                "{}",
                "Scan error(s) detected, terminate program.".purple()
            );
            return;
        }
    };

    for token in &tokens {
        println!("{:?}", token);
    }
    println!();
}
