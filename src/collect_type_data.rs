use std::collections::HashMap;

use crate::{
    func_compiler::FuncCompilerStack,
    parse_types::{Stmt, StmtType},
    value::ValueType,
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

