use crate::analysis_types::NatFuncData;

use std::collections::HashMap;

mod native_funcs;

pub fn register<'a>() -> HashMap<&'a str, NatFuncData> {
    let mut funcs = HashMap::new();

    native_funcs::register(&mut funcs);

    funcs
}
