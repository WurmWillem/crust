use crate::analysis_types::{NatFuncData, NatStructData};

use std::collections::HashMap;

mod funcs;
mod structs;
mod vec;

pub fn register<'a>() -> (HashMap<&'a str, NatFuncData>, HashMap<&'a str, NatStructData<'a>>) {
    let mut funcs = HashMap::new();
    let mut structs = HashMap::new();

    funcs::register(&mut funcs);
    structs::register(&mut structs);

    (funcs, structs)
}
