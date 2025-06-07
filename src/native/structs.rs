use std::collections::HashMap;

use crate::{
    analysis_types::{NatFuncData, NatStructData},
    heap::Heap,
    object::Object,
    value::{StackValue, ValueType},
};

pub fn register<'a>(structs: &mut HashMap<&'a str, NatStructData<'a>>) {
    let name = "Vector2";
    let fields = vec![(ValueType::Num, "x"), (ValueType::Num, "y")];

    let product = NatFuncData {
        parameters: vec![],
        func: vec2_product,
        return_ty: ValueType::Num,
    };

    let data = NatStructData {
        fields,
        methods: vec![("product", product)],
    };
    structs.insert(name, data);

    super::vec::register(structs);
}

fn vec2_product(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let StackValue::Obj(Object::Inst(inst)) = args[0] else {
        unreachable!()
    };
    let (x, y) = (inst.data.fields[0], inst.data.fields[1]);
    match (x, y) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1 * val2),
        _ => unreachable!(),
    }
}
