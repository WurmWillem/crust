use colored::Colorize;

pub const PRINT_SCAN_TOKENS: bool = false;
pub const DEBUG_TRACE_EXECUTION: bool = false;

pub fn print_error(line: u32, message: &str) {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    println!("{}{}{}{}", l, line, i, message);
}
