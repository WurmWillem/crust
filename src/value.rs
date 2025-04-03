use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    F64(f64),
    None,
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
                        panic!("Attempted to use operation that is not defined for this type.")
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
            StackValue::None => {
                panic!("Attempted to use operation that is not defined for this type.")
            }
        }
    }
}
impl Display for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::F64(value) => write!(f, "{:?}", value),
            StackValue::None => write!(f, "None"),
        }
    }
}
// enum ValueType {
//     Bool,
//     Num,
//     Nil,
// }
