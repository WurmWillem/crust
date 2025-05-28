use std::ops;
use std::ptr::NonNull;

use crate::chunk::Chunk;
use crate::value::StackValue;

pub struct Heap {
    // TODO: maybe add support for Table so you won't have to reallocate every time
    head: Option<Object>,
}
impl Heap {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn alloc<T, F>(&mut self, data: T, map: F) -> (Object, Gc<T>)
    where
        F: Fn(Gc<T>) -> Object,
    {
        let gc_data = Box::new(GcData {
            // marked: false,
            next: self.head,
            data,
        });

        let gc = Gc {
            ptr: NonNull::new(Box::into_raw(gc_data)).unwrap(),
        };

        let object = map(gc);

        self.head = Some(object);

        (object, gc)
    }

    unsafe fn dealloc(&mut self, object: Object) {
        match object {
            Object::Str(ptr) => {
                let raw = ptr.ptr.as_ptr();
                drop(Box::from_raw(raw));
            }
            Object::Func(ptr) => {
                let raw = ptr.ptr.as_ptr();
                drop(Box::from_raw(raw));
            }
            // WARN: I did not check if this actually works
            Object::Native(ptr) => {
                let raw = ptr.ptr.as_ptr();
                drop(Box::from_raw(raw));
            }
            Object::Arr(ptr) => {
                let raw = ptr.ptr.as_ptr();
                drop(Box::from_raw(raw));
            }
        }
    }
}
impl Drop for Heap {
    fn drop(&mut self) {
        let mut current = self.head.take();

        while let Some(object) = current {
            // WARN: I did not check if this actually works
            let next = match object {
                Object::Str(ref ptr) => ptr.next,
                Object::Func(ref ptr) => ptr.next,
                Object::Native(ref ptr) => ptr.next,
                Object::Arr(ref ptr) => ptr.next,
            };

            unsafe {
                self.dealloc(object);
            }

            current = next;
        }
    }
}

#[derive(Debug)]
pub struct Gc<T> {
    ptr: NonNull<GcData<T>>,
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
    // pub marked: bool,
    next: Option<Object>,
    pub data: T,
}

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Str(Gc<String>),
    Func(Gc<ObjFunc>),
    Native(Gc<ObjNative>),
    Arr(Gc<ObjArr>),
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

#[derive(Debug, Clone)]
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
