use std::{
    fmt,
    ops::{Neg, Not},
};

use crate::object::Object;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ValueType {
    None, // default value for locals
    Any,  // useful as generic type for functions like println()
    Null,
    Bool,
    Num,
    Str,
}
impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueType::None => unreachable!(),
            ValueType::Any => unreachable!(),
            ValueType::Null => write!(f, "Null"),
            ValueType::Bool => write!(f, "Bool"),
            ValueType::Num => write!(f, "Number"),
            ValueType::Str => write!(f, "String"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    Null,
    Bool(bool),
    F64(f64),
    Obj(Object),
}

macro_rules! add_num_operation {
    ($fun_name: ident, $op: tt) => {
        #[inline(always)]
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
        #[inline(always)]
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

    #[inline(always)]
    pub fn equals(self, rhs: StackValue) -> bool {
        match (self, rhs) {
            (StackValue::F64(lhs), StackValue::F64(rhs)) => lhs == rhs,
            (StackValue::Bool(lhs), StackValue::Bool(rhs)) => lhs == rhs,
            (StackValue::Null, StackValue::Null) => true,
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    pub fn and(self, rhs: StackValue) -> bool {
        match (self, rhs) {
            (StackValue::Bool(lhs), StackValue::Bool(rhs)) => lhs && rhs,
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    pub fn or(self, rhs: StackValue) -> bool {
        match (self, rhs) {
            (StackValue::Bool(lhs), StackValue::Bool(rhs)) => lhs || rhs,
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
impl StackValue {
    pub fn to_string(&self) -> String {
        match self {
            StackValue::Null => "null".to_string(),
            StackValue::Bool(b) => b.to_string(),
            StackValue::F64(f) => f.to_string(),
            StackValue::Obj(o) => match o {
                Object::Str(s) => format!("{}", s.data),
                Object::Func(_) => unreachable!(),
                Object::Native(_) => unreachable!(),
            },
        }
    }
    pub fn display(&self) -> String {
        match self {
            StackValue::Null => "null".to_string(),
            StackValue::Bool(b) => b.to_string(),
            StackValue::F64(f) => f.to_string(),
            StackValue::Obj(o) => match o {
                Object::Str(s) => format!("{:?}", s.data),
                Object::Func(f) => format!("fn {}", f.data.get_name()),
                Object::Native(f) => format!("nat {}", f.data.get_name()),
            },
        }
    }
}
