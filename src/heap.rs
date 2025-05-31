use crate::{
    error::PRINT_HEAP,
    object::{Gc, GcData, GcHeader, Object},
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
            // if let Object::Func(func) = object {
            //     println!("fn {}", func.data.get_name());
            // }

            let next = object.header().next;

            current = next;
        }
    }

    // fn blacken_obj(&mut self, obj: &Object) {
    //     match obj {
    //         Object::Str(_) => todo!(),
    //         Object::Func(obj) => (),
    //         Object::Native(_) => (),
    //         Object::Arr(arr) => {}
    //     }
    // }
    fn sweep(&mut self) {
        let mut new_head = None;
        let mut tail = &mut new_head;
        let mut current = self.head.take();

        while let Some(mut obj) = current {
            current = obj.take_next();

            if obj.is_marked() || matches!(obj, Object::Func(_)) || matches!(obj, Object::Native(_)) {
                obj.unmark();
                *tail = Some(obj);
                tail = &mut tail.as_mut().unwrap().header_mut().next;
            } else {
                dbg!("wow");
                unsafe { self.dealloc(obj) };
            }
        }

        self.head = new_head;
    }

    pub fn collect_garbage(&mut self, stack: &mut [StackValue; STACK_SIZE], stack_top: usize) {
        // return;
        for i in 0..stack_top {
            if let StackValue::Obj(mut obj) = stack[i] {
                obj.mark();
            }
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
            header: GcHeader {
                marked: false,
                next: self.head.take(),
            },
            data,
        });

        let gc = Gc {
            ptr: NonNull::new(Box::into_raw(gc_data)).unwrap(),
        };

        let object = map(gc);

        self.head = Some(object.clone());

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
        let mut current = self.head.take();

        while let Some(object) = current {
            let next = match object {
                Object::Str(ref ptr) => ptr.header.next,
                Object::Func(ref ptr) => ptr.header.next,
                Object::Native(ref ptr) => ptr.header.next,
                Object::Arr(ref ptr) => ptr.header.next,
            };

            unsafe {
                //dbg!(i);
                self.dealloc(object);
            }

            current = next;
        }
    }
}
