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

    let first = NatFuncData {
        parameters: vec![],
        func: first,
        return_ty: ValueType::Num,
    };
    let get = NatFuncData {
        parameters: vec![ValueType::Num],
        func: get,
        return_ty: ValueType::Num,
    };

    let data = NatStructData {
        fields,
        methods: vec![("first", first), ("get", get)],
    };
    structs.insert(name, data);
}

fn first(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let StackValue::Obj(Object::Inst(inst)) = args[0] else {
        unreachable!()
    };

    let arr = inst.data.fields[0];
    let StackValue::Obj(Object::Arr(arr)) = arr else {
        unreachable!()
    };

    let value = arr.data.values[0];
    value
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

    let value = arr.data.values[index as usize];
    value
    // StackValue::Null
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
