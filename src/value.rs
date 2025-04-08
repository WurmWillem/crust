use std::ops::{Neg, Not};

use crate::object::{Object, ObjectValue};

#[derive(Clone, Copy)]
pub enum StackValue {
    Null,
    Bool(bool),
    F64(f64),
    Obj(usize),
}

macro_rules! add_num_operation {
    ($fun_name: ident, $op: tt) => {
        pub fn $fun_name(self, rhs: StackValue) -> StackValue {
            match (self, rhs) {
                (StackValue::F64(lhs), StackValue::F64(rhs)) => StackValue::F64(lhs $op rhs),
                _ => unreachable!("$fun_name is only available for numbers"),
            }
        }
    };
}

macro_rules! add_num_comparison {
    ($fun_name: ident, $op: tt) => {
        pub fn $fun_name(self, rhs: StackValue) -> StackValue {
            match (self, rhs) {
                (StackValue::F64(lhs), StackValue::F64(rhs)) => StackValue::Bool(lhs $op rhs),
                _ => unreachable!("$fun_name is only available for numbers"),
            }
        }
    };
}

impl StackValue {
    // add_num_operation!(add_nums, +);
    add_num_operation!(sub_nums, -);
    add_num_operation!(mul_nums, *);
    add_num_operation!(div_nums, /);

    add_num_comparison!(is_greater_than, >);
    add_num_comparison!(is_greater_equal_than, >=);
    add_num_comparison!(is_less_than, <);
    add_num_comparison!(is_less_equal_than, <=);

    pub fn equals(self, rhs: StackValue) -> bool {
        match (self, rhs) {
            (StackValue::F64(lhs), StackValue::F64(rhs)) => lhs == rhs,
            (StackValue::Bool(lhs), StackValue::Bool(rhs)) => lhs == rhs,
            (StackValue::Null, StackValue::Null) => true,
            _ => unreachable!(),
        }
    }
}

impl Neg for StackValue {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        match self {
            StackValue::F64(value) => StackValue::F64(-value),
            _ => {
                unreachable!("Attempted to use operation that is not defined for this type.")
            }
        }
    }
}
impl Not for StackValue {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        match self {
            StackValue::Bool(value) => StackValue::Bool(!value),
            _ => {
                unreachable!("Attempted to use operation that is not defined for this type.")
            }
        }
    }
}
// trait DisplayWithContext {
//     fn fmt_with(&self, objects: &Vec<Object>)  -> String;
// }
//
// impl DisplayWithContext for StackValue {
//     fn fmt_with(&self, objects: &Vec<Object>) -> String {
//         self.display_with_context(objects)
//     }
// }
impl StackValue {
    pub fn display(&self, objects: &Vec<Object>) -> String {
        match self {
            StackValue::Null => "null".to_string(),
            StackValue::Bool(b) => b.to_string(),
            StackValue::F64(f) => f.to_string(),
            StackValue::Obj(idx) => {
                match &objects[*idx].value {
                    ObjectValue::Str(s) => format!("\"{}\"", s),
                    // handle other ObjectValue variants
                }
            }
        }
    }
}
// impl Display for StackValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// TODO: add print for object
//         match self {
//             StackValue::F64(value) => write!(f, "{:?}", value),
//             StackValue::Bool(value) => write!(f, "{:?}", value),
//             StackValue::Null => write!(f, "Null"),
//             // StackValue::Obj(_) => write!(f, "Null"),
//
//             StackValue::Obj(obj) => {
//                 let val = unsafe { (**obj).value.clone() };
//                 let str = match &val {
//                     ObjectValue::Str(str) => str,
//                 };
//                 write!(f, "{:?}", str)
//             },
//         }
//     }
// }
