use crate::analysis_types::{NatFuncHash, NatStructHash};

use std::collections::HashMap;

mod funcs;
mod structs;

pub fn register<'a>() -> (NatFuncHash<'a>, NatStructHash<'a>) {
    let mut funcs = HashMap::new();
    let mut structs = HashMap::new();

    funcs::register(&mut funcs);
    structs::register(&mut structs);

    (funcs, structs)
}
