use std::collections::HashMap;

use crate::{
    func_compiler::FuncCompilerStack, parse_types::{Stmt, StmtType}, value::ValueType
};

pub struct FuncData<'a> {
    // maybe name is unnecessary
    pub name: &'a str,
    pub parameters: Vec<(ValueType, &'a str)>,
    pub body: Stmt<'a>,
    pub return_ty: ValueType,
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

pub fn collect<'a>(stmts: &Vec<Stmt<'a>>) -> (HashMap<&'a str, FuncData<'a>>, FuncCompilerStack<'a>) {
    let mut func_data = HashMap::new();
    let mut comps = FuncCompilerStack::new();

    // pretty sure you should add local here as well

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
            func_data.insert(*name, data);

            
            comps.add_local(name, ValueType::None, stmt.line);
        }
    }
    (func_data, comps)
}
