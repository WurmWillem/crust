use std::ops;
use std::ptr::NonNull;

use crate::chunk::Chunk;
use crate::heap::Heap;
use crate::value::StackValue;

#[derive(Debug)]
pub struct Gc<T: GcMemSize> {
    pub ptr: NonNull<GcData<T>>,
}
impl<T: GcMemSize> Copy for Gc<T> {}
impl<T: GcMemSize> Clone for Gc<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: GcMemSize> ops::Deref for Gc<T> {
    type Target = GcData<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T: GcMemSize> ops::DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}
impl<T: GcMemSize> Gc<T> {
    fn header(&self) -> &GcHeader {
        unsafe { &self.ptr.as_ref().header }
    }

    fn header_mut(&mut self) -> &mut GcHeader {
        unsafe { &mut self.ptr.as_mut().header }
    }
}

pub trait GcMemSize {
    fn size_of(&self) -> usize;
}
impl GcMemSize for String {
    fn size_of(&self) -> usize {
        std::mem::size_of::<String>() + self.capacity()
    }
}

#[derive(Debug)]
pub struct GcHeader {
    pub marked: bool,
    pub next: Option<Object>,
}

#[derive(Debug)]
pub struct GcData<T> {
    pub header: GcHeader,
    pub data: T,
}

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Str(Gc<String>),
    Func(Gc<ObjFunc>),
    Native(Gc<ObjNative>),
    Arr(Gc<ObjArr>),
    Inst(Gc<ObjInstance>),
}
impl Object {
    pub fn header(&self) -> &GcHeader {
        match self {
            Object::Str(obj) => obj.header(),
            Object::Func(obj) => obj.header(),
            Object::Native(obj) => obj.header(),
            Object::Arr(obj) => obj.header(),
            Object::Inst(obj) => obj.header(),
        }
    }
    pub fn header_mut(&mut self) -> &mut GcHeader {
        match self {
            Object::Str(obj) => obj.header_mut(),
            Object::Func(obj) => obj.header_mut(),
            Object::Native(obj) => obj.header_mut(),
            Object::Arr(obj) => obj.header_mut(),
            Object::Inst(obj) => obj.header_mut(),
        }
    }
    pub fn is_marked(&self) -> bool {
        self.header().marked
    }
    pub fn unmark(&mut self) {
        self.header_mut().marked = false;
    }
    pub fn mark(&mut self) {
        self.header_mut().marked = true;
    }
    pub fn take_next(&mut self) -> Option<Object> {
        std::mem::take(&mut self.header_mut().next)
    }
}

#[derive(Debug, Clone)]
pub struct ObjArr {
    pub elements: Vec<StackValue>,
}
impl ObjArr {
    pub fn new(values: Vec<StackValue>) -> Self {
        Self { elements: values }
    }
}
impl GcMemSize for ObjArr {
    fn size_of(&self) -> usize {
        std::mem::size_of::<StackValue>() * self.elements.capacity()
    }
}

#[derive(Debug, Clone)]
pub struct ObjInstance {
    pub fields: Vec<StackValue>,
    pub methods: Vec<StackValue>,
}
impl ObjInstance {
    pub fn new(fields: Vec<StackValue>, methods: Vec<StackValue>) -> Self {
        Self { fields, methods }
    }
}
impl GcMemSize for ObjInstance {
    fn size_of(&self) -> usize {
        std::mem::size_of::<StackValue>() * self.fields.capacity()
            + std::mem::size_of::<StackValue>() * self.methods.capacity()
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
impl GcMemSize for ObjFunc {
    fn size_of(&self) -> usize {
        unreachable!()
    }
}

pub type NativeFunc = fn(&[StackValue], &mut Heap) -> StackValue;

#[derive(Debug, Clone)]
pub struct ObjNative {
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
impl GcMemSize for ObjNative {
    fn size_of(&self) -> usize {
        unreachable!()
    }
}
