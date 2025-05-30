use crate::{
    error::PRINT_HEAP,
    object::{Gc, GcData, Object},
    value::StackValue,
    vm::STACK_SIZE,
};
use std::ptr::NonNull;

const DEBUG_GC: bool = true;

pub struct Heap {
    // TODO: maybe add support for Table so you won't have to reallocate every time
    head: Option<Object>,
}
impl Heap {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn print(&self) {
        println!("start");
        let mut current = self.head;
        while let Some(object) = current {
            if let Object::Arr(arr) = object {
                print!("HEAP: [");
                for el in &arr.data.values {
                    print!("{}, ", el);
                }
                println!("]");
            }
            // WARN: I did not check if this actually works
            let next = match object {
                Object::Str(ref ptr) => ptr.next,
                Object::Func(ref ptr) => ptr.next,
                Object::Native(ref ptr) => ptr.next,
                Object::Arr(ref ptr) => ptr.next,
            };

            current = next;
        }
    }

    fn blacken_obj(&mut self, obj: &Object) {
        match obj {
            Object::Str(_) => todo!(),
            Object::Func(_) => (),
            Object::Native(_) => (),
            Object::Arr(arr) => {}
        }
    }

    fn sweep(&mut self) {
        let mut current = self.head;

        while let Some(mut object) = current {
            let next = match object {
                Object::Str(ptr) => ptr.next,
                Object::Func(ptr) => ptr.next,
                Object::Native(ptr) => ptr.next,
                Object::Arr(ptr) => ptr.next,
            };

            if !object.is_marked() {
                if let Object::Arr(arr) = object {
                    //dbg!("mark");
                    //dbg!(&arr.data);
                    dbg!("YOOO");
                    dbg!(&arr.data);
                    unsafe {
                        self.dealloc(object);
                    }
                }
            } else {
                object.unmark();
            }

            current = next;
        }
    }

    pub fn collect_garbage(&mut self, stack: &mut [StackValue; STACK_SIZE], stack_top: usize) {
        for i in 0..stack_top {
            stack[i].mark();
        }
        self.sweep();
        if PRINT_HEAP {
            self.print();
        }
    }

    pub fn alloc<T, F>(&mut self, data: T, map: F) -> (Object, Gc<T>)
    where
        F: Fn(Gc<T>) -> Object,
    {
        //self.collect_garbage();

        let gc_data = Box::new(GcData {
            marked: false,
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
        let mut i = 0;
        let mut current = self.head.take();

        while let Some(object) = current {
            i += 1;
            let next = match object {
                Object::Str(ref ptr) => ptr.next,
                Object::Func(ref ptr) => ptr.next,
                Object::Native(ref ptr) => ptr.next,
                Object::Arr(ref ptr) => ptr.next,
            };

            unsafe {
                //dbg!(i);
                self.dealloc(object);
            }

            current = next;
        }
    }
}
