use std::{fmt::Display, ops::Neg};

#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    F64(f64),
    None,
}
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
