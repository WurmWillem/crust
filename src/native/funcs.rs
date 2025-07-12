use std::collections::HashMap;

use crate::{analysis_types::NatFuncData, heap::Heap, object::Object, value::StackValue};

pub fn register(nat_funcs: &mut HashMap<&str, Vec<NatFuncData>>) {
    use crate::value::ValueType;

    macro_rules! add_func {
        ($name: expr, $func: ident, $parameters: expr, $return_ty: expr) => {
            let nat_func = NatFuncData {
                parameters: $parameters,
                func: $func,
                return_ty: $return_ty,
                use_self: false,
            };
            nat_funcs
                .entry($name)
                .or_insert_with(Vec::new)
                .push(nat_func);
        };
    }

    use ValueType as VT;
    add_func!("clock", clock, vec![], VT::F64);
    add_func!("print", print, vec![VT::Any], VT::Null);
    add_func!("println", println, vec![VT::Any], VT::Null);

    add_func!("sin", sin, vec![VT::F64], VT::F64);
    add_func!("cos", cos, vec![VT::F64], VT::F64);
    add_func!("tan", tan, vec![VT::F64], VT::F64);

    add_func!("min", min_f64, vec![VT::F64, VT::F64], VT::F64);
    add_func!("min", min_i64, vec![VT::I64, VT::I64], VT::I64);
    add_func!("min", min_u64, vec![VT::U64, VT::U64], VT::U64);

    add_func!("max", max_f64, vec![VT::F64, VT::F64], VT::F64);
    add_func!("max", max_i64, vec![VT::I64, VT::I64], VT::I64);
    add_func!("max", max_u64, vec![VT::U64, VT::U64], VT::U64);

    add_func!("abs", abs_f64, vec![VT::F64], VT::F64);
    add_func!("abs", abs_i64, vec![VT::I64], VT::I64);

    add_func!("sqrt", sqrt, vec![VT::F64], VT::F64);

    add_func!("pow", pow, vec![VT::F64, VT::F64], VT::F64);

    add_func!("len", len, vec![VT::Arr(Box::new(VT::Any))], VT::U64);
    add_func!("print_heap", print_heap, vec![], VT::Null);
}
// TODO: update these to work with all nums

fn clock(_args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    StackValue::F64(time.as_secs_f64())
}

fn print(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use colored::Colorize;

    let string = format!("{}", args[0]).green();
    print!("{string}");

    StackValue::Null
}
fn println(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use colored::Colorize;

    let string = format!("{}", args[0]).green();
    println!("{string}");

    StackValue::Null
}

fn sin(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sin())
    } else {
        unreachable!()
    }
}

fn cos(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.cos())
    } else {
        unreachable!()
    }
}

fn tan(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.tan())
    } else {
        unreachable!()
    }
}

fn min_f64(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.min(val2)),
        _ => unreachable!(),
    }
}
fn min_u64(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::U64(val1), StackValue::U64(val2)) => StackValue::U64(val1.min(val2)),
        _ => unreachable!(),
    }
}
fn min_i64(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::I64(val1), StackValue::I64(val2)) => StackValue::I64(val1.min(val2)),
        _ => unreachable!(),
    }
}
fn max_f64(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.max(val2)),
        _ => unreachable!(),
    }
}
fn max_u64(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::U64(val1), StackValue::U64(val2)) => StackValue::U64(val1.max(val2)),
        _ => unreachable!(),
    }
}
fn max_i64(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::I64(val1), StackValue::I64(val2)) => StackValue::I64(val1.max(val2)),
        _ => unreachable!(),
    }
}

fn abs_f64(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.abs())
    } else {
        unreachable!()
    }
}
fn abs_i64(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::I64(val) = val {
        StackValue::I64(val.abs())
    } else {
        unreachable!()
    }
}

fn sqrt(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sqrt())
    } else {
        unreachable!()
    }
}

fn pow(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.powf(val2)),
        _ => unreachable!(),
    }
}

fn len(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let arr = args[0];
    match arr {
        StackValue::Obj(Object::Arr(arr)) => StackValue::U64(arr.data.elements.len() as u64),
        _ => unreachable!(),
    }
}

fn print_heap(_args: &[StackValue], heap: &mut Heap) -> StackValue {
    heap.print();
    StackValue::Null
}
