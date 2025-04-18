// use crate::chunk::Chunk;
use std::ops;
use std::ptr::NonNull;

use crate::chunk::Chunk;

pub struct Heap {
    head: Option<Object>,
}
impl Heap {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn alloc<T, F>(&mut self, data: T, map: F) -> Object
    where
        F: Fn(Gc<T>) -> Object,
    {
        let gc_data = Box::new(GcData {
            marked: false,
            next: self.head.clone(),
            data,
        });

        let gc = Gc {
            ptr: NonNull::new(Box::into_raw(gc_data)).unwrap(),
        };

        let object = map(gc);

        self.head = Some(object.clone());

        object
    }

    unsafe fn dealloc(&mut self, object: Object) {
        match object {
            Object::Str(ptr) => {
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
            let next = match object {
                Object::Str(ref ptr) => ptr.next.clone(),
            };

            unsafe {
                self.dealloc(object);
            }
            
            current = next;
        }
    }
}

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
type RefFun = Gc<ObjFunc>;

#[derive(Debug, Clone)]
pub enum Object {
    Str(RefStr),
}

#[derive(Debug)]
pub struct ObjFunc {
    arity: u8,
    chunk: Chunk,
    pub name: String,
}
impl ObjFunc {
    pub fn new() -> Self {
        Self {
            arity: 0,
            chunk: Chunk::new(),
            name: "".to_string(),
        }
    }
}
