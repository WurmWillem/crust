use colored::Colorize;

pub fn print_error(line: usize, message: &str) {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    println!("{}{}{}{}", l, line, i, message);
}
