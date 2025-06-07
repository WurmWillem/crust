use crate::{
    analysis_types::{NatFuncData, NatStructData, NatStructHash},
    heap::Heap,
    value::{StackValue, ValueType},
};

pub fn register<'a>(structs: &mut NatStructHash) {
    let name = "Vec";
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
}

fn product(args: &[StackValue], _heap: &mut Heap) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    val1
    // match (val1, val2) {
    //     (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1 * val2),
    //     _ => unreachable!(),
    // }
}
