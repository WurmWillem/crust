use std::collections::HashMap;

use crate::native_funcs;
use crate::object::Heap;
use crate::object::ObjNative;
use crate::object::Object;
use crate::value::ValueType;
use crate::vm::MAX_FUNC_AMT;
use crate::StackValue;

#[derive(Debug)]
pub struct DeclaredTypes<'a> {
    funcs: Vec<DeclaredFunc<'a>>,
    structs: Vec<DeclaredStruct<'a>>,
}
impl<'a> DeclaredTypes<'a> {
    pub fn new(heap: &mut Heap) -> Self {
        let mut funcs = vec![];

        macro_rules! add_func {
            ($name: expr, $func: ident, $parameters: expr, $return_type: expr) => {
                let clock = ObjNative::new($name.to_string(), native_funcs::$func);
                let (clock, _) = heap.alloc(clock, Object::Native);
                let value = Some(StackValue::Obj(clock));
                let clock = DeclaredFunc::new($name, value, $parameters, $return_type);
                funcs.push(clock);
            };
        }
        add_func!("clock", clock, vec![], ValueType::Num);
        add_func!("print", print, vec![ValueType::Any], ValueType::Null);
        add_func!("println", println, vec![ValueType::Any], ValueType::Null);
        add_func!("sin", sin, vec![ValueType::Num], ValueType::Num);
        add_func!("cos", cos, vec![ValueType::Num], ValueType::Num);
        add_func!("tan", tan, vec![ValueType::Num], ValueType::Num);
        add_func!(
            "min",
            min,
            vec![ValueType::Num, ValueType::Num],
            ValueType::Num
        );
        add_func!(
            "max",
            max,
            vec![ValueType::Num, ValueType::Num],
            ValueType::Num
        );
        add_func!("abs", abs, vec![ValueType::Num], ValueType::Num);
        add_func!("sqrt", sqrt, vec![ValueType::Num], ValueType::Num);
        add_func!(
            "pow",
            pow,
            vec![ValueType::Num, ValueType::Num],
            ValueType::Num
        );

        Self {
            funcs,
            structs: Vec::new(),
        }
    }

    pub fn to_stack_value_arr(self) -> [StackValue; MAX_FUNC_AMT] {
        let mut arr = [StackValue::Null; MAX_FUNC_AMT];
        for i in 0..self.funcs.len() {
            if let Some(val) = self.funcs[i].value {
                arr[i] = val;
            }
        }
        arr
    }

    pub fn add_struct(&mut self, name: &'a str, fields: HashMap<&'a str, u8>) {
        let str = DeclaredStruct::new(name, fields);
        self.structs.push(str);
    }

    pub fn add_func(&mut self, name: &'a str, parameters: Vec<ValueType>, return_type: ValueType) {
        let func = DeclaredFunc::new_partial(name, parameters, return_type);
        self.funcs.push(func);
    }

    pub fn patch_value(&mut self, value: StackValue) {
        self.funcs.last_mut().unwrap().value = Some(value);
    }

    pub fn resolve_func(&self, name: &str) -> Option<(u8, Vec<ValueType>, ValueType)> {
        for i in 0..self.funcs.len() {
            if self.funcs[i].name == name {
                let parameters = self.funcs[i].parameters.clone();
                let return_type = self.funcs[i].return_type;
                return Some((i as u8, parameters, return_type));
            }
        }
        None
    }
}

#[derive(Debug)]
struct DeclaredStruct<'a> {
    name: &'a str,
    fields: HashMap<&'a str, u8>,
}
impl<'a> DeclaredStruct<'a> {
    fn new(name: &'a str,  fields: HashMap<&'a str, u8>) -> Self {
        Self {
            name,
            fields,
        }
    }
}

#[derive(Debug)]
struct DeclaredFunc<'a> {
    name: &'a str,
    value: Option<StackValue>,
    parameters: Vec<ValueType>,
    return_type: ValueType,
}
impl<'a> DeclaredFunc<'a> {
    fn new(
        name: &'a str,
        value: Option<StackValue>,
        parameters: Vec<ValueType>,
        return_type: ValueType,
    ) -> Self {
        Self {
            name,
            value,
            parameters,
            return_type,
        }
    }
    fn new_partial(name: &'a str, parameters: Vec<ValueType>, return_type: ValueType) -> Self {
        Self {
            name,
            value: None,
            parameters,
            return_type,
        }
    }
}
