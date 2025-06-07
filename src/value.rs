use std::{
    fmt::{self, Display},
    ops::{Neg, Not},
};

use crate::object::Object;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ValueType {
    None, // default value for locals
    Any,  // useful as generic type for functions like println()
    Null,
    Bool,
    Num,
    Str,
    Arr(Box<ValueType>),
    Struct(String),
}
impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueType::None => unreachable!(),
            ValueType::Arr(ty) => write!(f, "[{}]", ty),
            ValueType::Any => write!(f, "Any"),
            ValueType::Null => write!(f, "Null"),
            ValueType::Bool => write!(f, "Bool"),
            ValueType::Num => write!(f, "Number"),
            ValueType::Str => write!(f, "String"),
            ValueType::Struct(s) => write!(f, "struct {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    Null,
    Bool(bool),
    F64(f64),
    U64(u64),
    I64(i64),
    Obj(Object),
}

macro_rules! add_num_operation {
    ($fun_name: ident, $op: tt) => {
        #[inline(always)]
        pub fn $fun_name(self, rhs: StackValue) -> StackValue {
            match (self, rhs) {
                (StackValue::F64(lhs), StackValue::F64(rhs)) => StackValue::F64(lhs $op rhs),
                (StackValue::I64(lhs), StackValue::I64(rhs)) => StackValue::I64(lhs $op rhs),
                (StackValue::U64(lhs), StackValue::U64(rhs)) => StackValue::U64(lhs $op rhs),
                (StackValue::U64(lhs), StackValue::I64(rhs)) => StackValue::I64(lhs as i64 $op rhs),
                (StackValue::I64(lhs), StackValue::U64(rhs)) => StackValue::I64(lhs $op rhs as i64),
                _ => unreachable!("operation is only available for numbers"),
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
                (StackValue::I64(lhs), StackValue::I64(rhs)) => StackValue::Bool(lhs $op rhs),
                (StackValue::U64(lhs), StackValue::U64(rhs)) => StackValue::Bool(lhs $op rhs),
                (StackValue::I64(lhs), StackValue::U64(rhs)) => StackValue::Bool(lhs $op rhs as i64),
                (StackValue::U64(lhs), StackValue::I64(rhs)) => StackValue::Bool((lhs as i64) $op rhs),
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
            (StackValue::I64(lhs), StackValue::I64(rhs)) => lhs == rhs,
            (StackValue::U64(lhs), StackValue::U64(rhs)) => lhs == rhs,
            (StackValue::I64(lhs), StackValue::U64(rhs)) => lhs == rhs as i64,
            (StackValue::U64(lhs), StackValue::I64(rhs)) => lhs as i64 == rhs,
            (StackValue::Bool(lhs), StackValue::Bool(rhs)) => lhs == rhs,
            (StackValue::Null, StackValue::Null) => true,
            (StackValue::Obj(Object::Str(str1)), StackValue::Obj(Object::Str(str2))) => {
                str1.data == str2.data
            }
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
            StackValue::I64(value) => StackValue::I64(-value),
            StackValue::U64(_) => panic!("attempted to use minus on unsigned int."),
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
impl Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackValue::Null => write!(f, "null"),
            StackValue::Bool(b) => write!(f, "{}", b),
            StackValue::F64(num) => write!(f, "{}", num),
            StackValue::U64(num) => write!(f, "{}", num),
            StackValue::I64(num) => write!(f, "{}", num),
            StackValue::Obj(o) => match o {
                Object::Str(s) => write!(f, "{}", s.data),
                Object::Func(_) => unreachable!(),
                Object::Native(_) => unreachable!(),
                Object::Arr(a) => write!(f, "{:?}", a.data.elements),
                Object::Inst(i) => write!(f, "inst {:?}", i.data.fields),
            },
        }
    }
}
impl StackValue {
    pub fn display(&self) -> String {
        match self {
            StackValue::Null => "null".to_string(),
            StackValue::Bool(b) => b.to_string(),
            StackValue::F64(f) => f.to_string(),
            StackValue::U64(f) => f.to_string(),
            StackValue::I64(f) => f.to_string(),
            StackValue::Obj(o) => match o {
                Object::Str(s) => format!("{:?}", s.data),
                Object::Func(f) => format!("fn {}", f.data.get_name()),
                Object::Native(f) => format!("nat {}", f.data.get_name()),
                Object::Arr(a) => format!("arr {:?}", a.data.elements),
                Object::Inst(i) => format!("inst {:?}", i.data.fields),
            },
        }
    }
}
