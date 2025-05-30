use std::ops;
use std::ptr::NonNull;

use crate::chunk::Chunk;
use crate::value::StackValue;

#[derive(Debug)]
pub struct Gc<T> {
    pub ptr: NonNull<GcData<T>>,
}
impl<T> Copy for Gc<T> {}
impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        *self
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

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Str(Gc<String>),
    Func(Gc<ObjFunc>),
    Native(Gc<ObjNative>),
    Arr(Gc<ObjArr>),
}
impl Object {
    pub fn is_marked(&self) -> bool {
        match self {
            Object::Str(obj) => obj.marked,
            Object::Func(obj) => obj.marked,
            Object::Native(obj) => obj.marked,
            Object::Arr(obj) => obj.marked,
        }
    }
    pub fn unmark(&mut self) {
        match self {
            Object::Str(obj) => obj.marked = false,
            Object::Func(obj) => obj.marked = false,
            Object::Native(obj) => obj.marked = false,
            Object::Arr(obj) => obj.marked = false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjArr {
    pub values: Vec<StackValue>,
}
impl ObjArr {
    pub fn new(values: Vec<StackValue>) -> Self {
        Self { values }
    }
}
impl std::fmt::Display for ObjArr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.values)
    }
}

#[derive(Debug)]
pub struct ObjFunc {
    pub chunk: Chunk,
    name: String,
}
impl ObjFunc {
    pub fn new(name: String) -> Self {
        Self {
            chunk: Chunk::new(),
            name,
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

pub type NativeFunc = fn(&[StackValue]) -> StackValue;

#[derive(Debug, Clone)]
pub struct ObjNative {
    // TODO: maybe this name actually isn't necessary, cuz DeclaredFunc has it too
    name: String,
    pub func: NativeFunc,
}
impl ObjNative {
    pub fn new(name: String, func: NativeFunc) -> Self {
        Self { name, func }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
