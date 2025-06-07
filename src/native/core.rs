use crate::{analysis_types::NatFuncData, heap::Heap, object::Object, value::StackValue};
use std::collections::HashMap;

pub fn register<'a>(nat_funcs: &mut HashMap<&'a str, NatFuncData>) {
    use crate::value::ValueType;

    macro_rules! add_func {
        ($name: expr, $func: ident, $parameters: expr, $return_ty: expr) => {
            let nat_func = NatFuncData {
                parameters: $parameters,
                func: $func,
                return_ty: $return_ty,
            };
            nat_funcs.insert($name, nat_func);
        };
    }

    use ValueType as VT;
    add_func!("clock", clock, vec![], VT::Num);
    add_func!("print", print, vec![VT::Any], VT::Null);
    add_func!("println", println, vec![VT::Any], VT::Null);
    add_func!("sin", sin, vec![VT::Num], VT::Num);
    add_func!("cos", cos, vec![VT::Num], VT::Num);
    add_func!("tan", tan, vec![VT::Num], VT::Num);
    add_func!("min", min, vec![VT::Num, VT::Num], VT::Num);
    add_func!("max", max, vec![VT::Num, VT::Num], VT::Num);
    add_func!("abs", abs, vec![VT::Num], VT::Num);
    add_func!("sqrt", sqrt, vec![VT::Num], VT::Num);
    add_func!("pow", pow, vec![VT::Num, VT::Num], VT::Num);
    add_func!("len", len, vec![VT::Arr(Box::new(VT::Any))], VT::Num);
    add_func!("print_heap", print_heap, vec![], VT::Null);
    add_func!(
        "push",
        push,
        vec![VT::Arr(Box::new(VT::Any)), VT::Any],
        VT::Null
    );
}

fn clock(_args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    StackValue::F64(time.as_secs_f64())
}

fn print(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use colored::Colorize;

    let string = format!("{}", args[0]).green();
    print!("{}", string);

    StackValue::Null
}
fn println(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use colored::Colorize;

    let string = format!("{}", args[0]).green();
    println!("{}", string);

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

fn min(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.min(val2)),
        _ => unreachable!(),
    }
}

fn max(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.max(val2)),
        _ => unreachable!(),
    }
}

fn abs(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.abs())
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
        StackValue::Obj(Object::Arr(arr)) => StackValue::F64(arr.data.values.len() as f64),
        _ => unreachable!(),
    }
}

fn print_heap(_args: &[StackValue], heap: &mut Heap) -> StackValue {
    heap.print();
    StackValue::Null
}

fn push(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let arr = args[0];
    if let StackValue::Obj(Object::Arr(mut arr)) = arr {
        arr.data.values.push(args[1]);
    } else {
        unreachable!()
    }
    StackValue::Null
}
