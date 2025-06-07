use crate::{
    analysis_types::{NatFuncData, NatStructData, NatStructHash},
    heap::Heap,
    object::Object,
    value::{StackValue, ValueType},
};

pub fn register<'a>(structs: &mut NatStructHash) {
    let name = "Vec2";
    let fields = vec![(ValueType::Num, "x"), (ValueType::Num, "y")];

    let product = NatFuncData {
        parameters: vec![],
        func: product,
        return_ty: ValueType::Num,
    };

    let data = NatStructData {
        fields,
        methods: vec![("product", product)],
    };
    structs.insert(name, data);

    let name = "Vec";
    let field_ty = ValueType::Arr(Box::new(ValueType::Any));
    let fields = vec![(field_ty, "elements")];

    let get = NatFuncData {
        parameters: vec![ValueType::Num],
        func: get,
        return_ty: ValueType::Num,
    };
    let push = NatFuncData {
        parameters: vec![ValueType::Num],
        func: push,
        return_ty: ValueType::Null,
    };
    let print = NatFuncData {
        parameters: vec![],
        func: print,
        return_ty: ValueType::Null,
    };

    let data = NatStructData {
        fields,
        methods: vec![("get", get), ("push", push), ("print", print)],
    };
    structs.insert(name, data);
}

fn get(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let StackValue::Obj(Object::Inst(inst)) = args[0] else {
        unreachable!()
    };

    let arr = inst.data.fields[0];
    let StackValue::Obj(Object::Arr(arr)) = arr else {
        unreachable!()
    };

    let StackValue::F64(index) = args[1] else {
        unreachable!()
    };

    let value = arr.data.elements[index as usize];
    value
}

fn push(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let StackValue::Obj(Object::Inst(inst)) = args[0] else {
        unreachable!()
    };

    let arr = inst.data.fields[0];
    let StackValue::Obj(Object::Arr(mut arr)) = arr else {
        unreachable!()
    };

    arr.data.elements.push(args[1]);
    StackValue::Null
}

fn print(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    use colored::Colorize;

    let StackValue::Obj(Object::Inst(inst)) = args[0] else {
        unreachable!()
    };

    let arr = inst.data.fields[0];
    let StackValue::Obj(Object::Arr(arr)) = arr else {
        unreachable!()
    };

    let els = &arr.data.elements;
    print!("[");
    if !els.is_empty() {
        let string = format!("{}", els[0]).green();
        print!("{}", string);
        
        for el in arr.data.elements.iter().skip(1) {
            let string = format!(", {}", el).green();
            print!("{}", string);
        }
    }
    println!("]");

    StackValue::Null
}

fn product(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let StackValue::Obj(Object::Inst(inst)) = args[0] else {
        unreachable!()
    };
    let (x, y) = (inst.data.fields[0], inst.data.fields[1]);
    match (x, y) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1 * val2),
        _ => unreachable!(),
    }
}
