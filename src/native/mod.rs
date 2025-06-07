use crate::analysis_types::NatFuncData;

use std::collections::HashMap;

mod core;
pub fn register<'a>() -> HashMap<&'a str, NatFuncData> {
    let mut funcs = HashMap::new();

    core::register(&mut funcs);

    funcs
}
