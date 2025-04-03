use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum Value {
    F64(f64),
    None,
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::F64(value) => write!(f, "{:?}", value),
            Value::None => write!(f, "None"),
        }
    }
}
// enum ValueType {
//     Bool,
//     Num,
//     Nil,
// }

