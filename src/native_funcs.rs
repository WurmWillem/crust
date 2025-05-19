use crate::value::StackValue;

pub fn clock(_args: &[StackValue]) -> StackValue {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    StackValue::F64(time.as_secs_f64())
}

pub fn print(args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    print!("{}", string);

    StackValue::Null
}
pub fn println(args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    println!("{}", string);

    StackValue::Null
}
