use crate::object::Heap;
use crate::object::Object;
use crate::value::StackValue;

pub fn clock(_heap: &Heap, _args: &[StackValue]) -> StackValue {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    StackValue::F64(time.as_secs_f64())
}

pub fn print(_heap: &Heap, args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    print!("{}", string);

    StackValue::Null
}
pub fn println(_heap: &Heap, args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    println!("{}", string);

    StackValue::Null
}

pub fn sin(_heap: &Heap, args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sin())
    } else {
        unreachable!()
    }
}

pub fn cos(_heap: &Heap, args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.cos())
    } else {
        unreachable!()
    }
}

pub fn tan(_heap: &Heap, args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.tan())
    } else {
        unreachable!()
    }
}

pub fn min(_heap: &Heap, args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.min(val2)),
        _ => unreachable!(),
    }
}

pub fn max(_heap: &Heap, args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.max(val2)),
        _ => unreachable!(),
    }
}

pub fn abs(_heap: &Heap, args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.abs())
    } else {
        unreachable!()
    }
}

pub fn sqrt(_heap: &Heap, args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sqrt())
    } else {
        unreachable!()
    }
}

pub fn pow(_heap: &Heap, args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.powf(val2)),
        _ => unreachable!(),
    }
}

pub fn readfile(heap: &mut Heap, args: &[StackValue]) -> StackValue {
    use colored::Colorize;
    use std::fs;

    let file = &args[0].display();

    let file_contents = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(_e) => {
            let msg = format!("Error reading file: {}", file).red();
            println!("{}", msg);
            String::new()
        }
    };

    let (object, _) = heap.alloc(file_contents, Object::Str);

    StackValue::Obj(object)
}

// readfile alternative
// pub fn readfile(heap: &mut Heap, args: &[StackValue]) -> StackValue {
//     use std::fs;
//
//     let file = &args[0].display();
//
//     match fs::read_to_string(file) {
//         Ok(contents) => {
//             let (object, _) = heap.alloc(contents, Object::Str);
//             StackValue::Obj(object)
//         }
//         Err(_e) => StackValue::Null,
//     }
// }
