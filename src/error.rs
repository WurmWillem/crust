use colored::Colorize;

pub const PRINT_SCAN_TOKENS: bool = false;
pub const DEBUG_TRACE_EXECUTION: bool = true;

pub const EXPECTED_SEMICOLON_MSG: &str = "Expected ';' at end of statement.";

pub struct ParseError {
    pub msg: String,
    pub line: u32,
}
impl ParseError {
    pub fn new(line: u32, msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            line,
        }
    }
}

pub fn print_error(line: u32, message: &str) {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    println!("{}{}{}{}", l, line, i, message);
}
