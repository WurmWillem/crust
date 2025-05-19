use crate::native_funcs;
use crate::object::Heap;
use crate::object::ObjNative;
use crate::object::Object;
use crate::value::ValueType;
use crate::vm::MAX_FUNC_AMT;
use crate::StackValue;

pub struct DeclaredFuncStack<'a> {
    funcs: [DeclaredFunc<'a>; MAX_FUNC_AMT],
    top: usize,
}
impl<'a> DeclaredFuncStack<'a> {
    pub fn new(heap: &mut Heap) -> Self {
        let mut funcs = std::array::from_fn(|_| DeclaredFunc::new_partial(None));
        let mut i = 0;

        macro_rules! add_func {
            ($name: expr, $func: ident, $parameters: expr, $return_type: expr) => {
                let clock = ObjNative::new($name.to_string(), native_funcs::$func);
                let (clock, _) = heap.alloc(clock, Object::Native);
                let value = Some(StackValue::Obj(clock));
                let clock = DeclaredFunc::new($name, value, $parameters, $return_type);
                funcs[i] = clock;
                i += 1;
            };
        }
        add_func!("clock", clock, vec![], ValueType::Num);
        add_func!("println", println, vec![ValueType::Any], ValueType::Null);
        add_func!("sin", sin, vec![ValueType::Num], ValueType::Num);
        add_func!("cos", cos, vec![ValueType::Num], ValueType::Num);
        add_func!("tan", tan, vec![ValueType::Num], ValueType::Num);
        add_func!("print", print, vec![ValueType::Any], ValueType::Null);

        Self { funcs, top: i }
    }

    pub fn to_stack_value_arr(self) -> [StackValue; MAX_FUNC_AMT] {
        let mut arr = [StackValue::Null; MAX_FUNC_AMT];
        for i in 0..=self.top {
            if let Some(val) = self.funcs[i].value {
                arr[i] = val;
            }
        }
        arr
    }

    pub fn patch_func(
        &mut self,
        name: &'a str,
        parameters: Vec<ValueType>,
        return_type: ValueType,
    ) {
        self.funcs[self.top].name = name;
        self.funcs[self.top].parameters = parameters;
        self.funcs[self.top].return_type = return_type;
    }

    pub fn edit_value_and_increment_top(&mut self, value: StackValue) {
        self.funcs[self.top].value = Some(value);
        self.top += 1;
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

#[derive(Debug, Clone)]
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
    fn new_partial(value: Option<StackValue>) -> Self {
        Self {
            name: "",
            value,
            parameters: Vec::new(),
            return_type: ValueType::Null,
        }
    }
}
