use std::collections::HashMap;

use crate::{
    parse_types::{Stmt, StmtType},
    value::ValueType,
};

pub struct FuncData<'a> {
    // maybe name is unnecessary
    name: &'a str,
    parameters: Vec<(ValueType, &'a str)>,
    body: Stmt<'a>,
    return_ty: ValueType,
}
impl<'a> FuncData<'a> {
    fn new(
        name: &'a str,
        parameters: Vec<(ValueType, &'a str)>,
        body: Stmt<'a>,
        return_ty: ValueType,
    ) -> Self {
        Self {
            name,
            parameters,
            body,
            return_ty,
        }
    }
}

pub fn collect<'a>(stmts: &Vec<Stmt>) -> HashMap<&'a str, FuncData<'a>> {
    let mut func_data = HashMap::new();

    for stmt in stmts {
        if let StmtType::Func {
            name,
            parameters,
            body,
            return_ty,
        } = &stmt.stmt
        {
            let body = (**body).clone();
            let data = FuncData::new(name, parameters.clone(), body, *return_ty);
            func_data.insert(name, data);
        }
    }
    todo!();
}
