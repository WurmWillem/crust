use crate::{heap::Heap, object::Object, value::StackValue};

pub fn clock(_args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    StackValue::F64(time.as_secs_f64())
}

pub fn print(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    print!("{}", string);

    StackValue::Null
}
pub fn println(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    println!("{}", string);

    StackValue::Null
}

pub fn sin(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sin())
    } else {
        unreachable!()
    }
}

pub fn cos(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.cos())
    } else {
        unreachable!()
    }
}

pub fn tan(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.tan())
    } else {
        unreachable!()
    }
}

pub fn min(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.min(val2)),
        _ => unreachable!(),
    }
}

pub fn max(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.max(val2)),
        _ => unreachable!(),
    }
}

pub fn abs(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.abs())
    } else {
        unreachable!()
    }
}

pub fn sqrt(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sqrt())
    } else {
        unreachable!()
    }
}

pub fn pow(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.powf(val2)),
        _ => unreachable!(),
    }
}

pub fn len(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let arr = args[0];
    match arr {
        StackValue::Obj(Object::Arr(arr)) => StackValue::F64(arr.data.values.len() as f64),
        _ => unreachable!(),
    }
}

pub fn print_heap(args: &[StackValue], heap: &mut Heap) -> StackValue {
    heap.print();
    StackValue::Null
}
