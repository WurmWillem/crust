use crate::analysis_types::{StructData, StructHash};

pub fn register<'a>(nat_funcs: &mut StructHash) {
    let name = "Vec";
    let fields = vec![];
    let data = StructData::new(fields);
    nat_funcs.insert(name, data);
}
