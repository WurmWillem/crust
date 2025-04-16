use crate::chunk::Chunk;

#[derive(Debug, )]
pub struct Object {
    pub value: ObjectValue,
}

#[derive(Debug)]
pub enum ObjectValue {
    Str(String),
    Func(ObjFunction),
}

#[derive(Debug)]
pub struct ObjFunction {
    arity: u8,
    chunk: Chunk,
    pub name: String,
}
impl ObjFunction {
    pub fn new() -> Self {
        Self { arity: 0, chunk: Chunk::new(), name: "".to_string() }
    }
    
}
// #[repr(C)]
// pub struct Object {
//     kind: ObjType,
// }
// impl Object {
//     pub fn new(kind: ObjType) -> Self {
//         Object { kind }
//     }
// }

// #[repr(C)]
// #[derive(Debug, Clone)]
// pub enum ObjType {
//     String,
// }

// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct ObjString {
//     kind: ObjType,
//     pub value: String,
// }
// impl ObjString {
//    pub fn new(kind: ObjType, value: String) -> Self {
//        Self { kind, value, }
//     }
// }
