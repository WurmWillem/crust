use crate::value::StackValue;

pub fn clock(_args: &[StackValue]) -> StackValue {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    StackValue::F64(time.as_secs_f64())
}

pub fn println(args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    println!("{}", string);

    StackValue::Null
}

pub fn sin(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sin())
    } else {
        unreachable!()
    }
}

pub fn cos(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.cos())
    } else {
        unreachable!()
    }
}

pub fn tan(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.tan())
    } else {
        unreachable!()
    }
}
