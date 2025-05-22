use crate::value::StackValue;

pub fn clock(_args: &[StackValue]) -> StackValue {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    StackValue::F64(time.as_secs_f64())
}

pub fn print(args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].to_string().green();
    print!("{}", string);

    StackValue::Null
}
pub fn println(args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].to_string().green();
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

pub fn min(args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.min(val2)),
        _ => unreachable!(),
    }
}

pub fn max(args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.max(val2)),
        _ => unreachable!(),
    }
}

pub fn abs(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.abs())
    } else {
        unreachable!()
    }
}

pub fn sqrt(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sqrt())
    } else {
        unreachable!()
    }
}

pub fn pow(args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.powf(val2)),
        _ => unreachable!(),
    }
}
