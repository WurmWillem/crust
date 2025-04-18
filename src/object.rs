// use crate::chunk::Chunk;
use std::ops;
use std::ptr::NonNull;

// TODO: look into making fields private

#[derive(Debug, Copy)]
pub struct Gc<T> {
    pub ptr: NonNull<GcData<T>>,
}
impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Gc { ptr: self.ptr }
    }
}
impl<T> ops::Deref for Gc<T> {
    type Target = GcData<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T> ops::DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

#[derive(Debug)]
pub struct GcData<T> {
    pub marked: bool,
    pub next: Option<Object>,
    pub data: T,
}

type RefStr = Gc<String>;

#[derive(Debug, Clone)]
pub enum Object {
    Str(RefStr),
}

#[derive(Debug)]
pub struct ObjString {
    pub chars: String,
    pub hash: u32,
}
//
// #[derive(Debug)]
// pub struct ObjFunction {
//     arity: u8,
//     chunk: Chunk,
//     pub name: String,
// }
// impl ObjFunction {
//     pub fn new() -> Self {
//         Self {
//             arity: 0,
//             chunk: Chunk::new(),
//             name: "".to_string(),
//         }
//     }
// }
