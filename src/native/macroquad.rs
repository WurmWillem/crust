use crate::{analysis_types::NatFuncData, heap::Heap, value::StackValue};
use std::collections::HashMap;
use macroquad::prelude::*;

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
    // add_func!("println", println, vec![VT::Any], VT::Null);
}

// fn println(args: &[StackValue], _heap: &mut Heap) -> StackValue {
//     use colored::Colorize;
//
//     let string = format!("{}", args[0]).green();
//     println!("{}", string);
//
//     StackValue::Null
// }

