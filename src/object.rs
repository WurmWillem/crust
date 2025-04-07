
#[repr(C)]
#[derive(Clone, Copy)]
pub union Object {
    pub str: *mut ObjString,
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

#[repr(C)]
#[derive(Debug, Clone)]
pub enum ObjType {
    String,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ObjString {
    kind: ObjType,
    pub value: String,
}
impl ObjString {
   pub fn new(kind: ObjType, value: String) -> Self {
       Self { kind, value, }
    } 
}
