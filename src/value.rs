use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    Null,
    Bool(bool),
    F64(f64),
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
    add_num_operation!(add_nums, +);
    add_num_operation!(sub_nums, -);
    add_num_operation!(mul_nums, *);
    add_num_operation!(div_nums, /);

    add_num_comparison!(equal, ==);
    add_num_comparison!(greater_than, >);
    add_num_comparison!(greater_equal_than, >=);
    add_num_comparison!(less_than, <);
    add_num_comparison!(less_equal_than, <=);
}

macro_rules! add_op_overload {
    ($op_name: ident, $fun_name: ident, $op: tt) => {
        impl $op_name for StackValue {
            type Output = Self;

            #[inline(always)]
            fn $fun_name(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (StackValue::F64(lhs), StackValue::F64(rhs)) => StackValue::F64(lhs $op rhs),
                    _ => {
                        unreachable!("Attempted to use operation that is not defined for this type.")
                    }
                }
            }
        }
    };
}
add_op_overload!(Add, add, +);
add_op_overload!(Sub, sub, -);
add_op_overload!(Mul, mul, *);
add_op_overload!(Div, div, /);

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

impl Display for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::F64(value) => write!(f, "{:?}", value),
            StackValue::Bool(value) => write!(f, "{:?}", value),
            StackValue::Null => write!(f, "Null"),
        }
    }
}
