use crate::{
    error::PRINT_HEAP,
    object::{Gc, GcData, GcHeader, GcMemSize, Object},
    value::StackValue,
    vm::STACK_SIZE,
};
use std::ptr::NonNull;

const INITIAL_GC_THRESHOLD: usize = 1024 * 1024 * 10;

pub struct Heap {
    // TODO: maybe add support for Table so we won't have to reallocate every time
    head: Option<Object>,
    permanent_head: Option<Object>,
    bytes_allocated: usize,
    gc_threshold: f64,
}
impl Heap {
    pub fn new() -> Self {
        Self {
            head: None,
            permanent_head: None,
            bytes_allocated: 0,
            gc_threshold: INITIAL_GC_THRESHOLD as f64,
        }
    }
    pub fn print(&self) {
        println!("start");
        let mut current = self.head;
        while let Some(object) = current {
            if let Object::Arr(arr) = object {
                print!("HEAP: [");
                for el in &arr.data.elements {
                    print!("{}, ", el.display());
                }
                println!("]");
            }
            // if let Object::Func(func) = object {
            //     println!("fn {}", func.data.get_name());
            // }
            if let Object::Str(str) = object {
                println!("str {}", str.data);
            }

            let next = object.header().next;

            current = next;
        }
    }

    fn trace_objects(&mut self, gray_list: &mut Vec<Object>) {
        while let Some(obj) = gray_list.pop() {
            self.blacken_obj(obj, gray_list);
        }
    }

    fn mark_object(&mut self, mut obj: Object, gray_list: &mut Vec<Object>) {
        if !obj.is_marked() {
            obj.mark();
            gray_list.push(obj);
        }
    }

    fn blacken_obj(&mut self, obj: Object, gray_list: &mut Vec<Object>) {
        match obj {
            Object::Str(_) => (),
            Object::Func(_) => (),
            Object::Native(_) => (),
            Object::Inst(_) => todo!(),
            Object::Arr(arr) => {
                for el in &arr.data.elements {
                    if let StackValue::Obj(obj) = el {
                        self.mark_object(*obj, gray_list);
                    }
                }
            }
        }
    }
    fn sweep(&mut self) {
        let mut new_head = None;
        let mut tail = &mut new_head;
        let mut current = self.head.take();

        while let Some(mut obj) = current {
            current = obj.take_next();

            if obj.is_marked() {
                obj.unmark();
                *tail = Some(obj);
                tail = &mut tail.as_mut().unwrap().header_mut().next;
            } else {
                unsafe { self.dealloc(obj) };
            }
        }

        self.head = new_head;
    }

    pub fn collect_garbage(&mut self, stack: &mut [StackValue; STACK_SIZE], stack_top: usize) {
        let mut gray_objects = vec![];
        for value in stack.iter().take(stack_top) {
            if let StackValue::Obj(obj) = value {
                self.mark_object(*obj, &mut gray_objects);
            }
        }

        self.trace_objects(&mut gray_objects);
        self.sweep();

        if PRINT_HEAP {
            self.print();
        }
    }

    pub fn alloc<T: GcMemSize, F>(
        &mut self,
        data: T,
        map: F,
        stack: &mut [StackValue; STACK_SIZE],
        stack_top: usize,
    ) -> (Object, Gc<T>)
    where
        F: Fn(Gc<T>) -> Object,
    {
        // TODO: add realloc()
        //self.collect_garbage();
        let size = data.size_of() + std::mem::size_of::<GcData<T>>();
        self.bytes_allocated += size;

        if self.bytes_allocated > self.gc_threshold as usize {
            self.collect_garbage(stack, stack_top);
            self.bytes_allocated = 0;
            self.gc_threshold *= 1.8;
        }

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

        self.head = Some(object);

        (object, gc)
    }

    pub fn alloc_permanent<T: GcMemSize, F>(&mut self, data: T, map: F) -> (Object, Gc<T>)
    where
        F: Fn(Gc<T>) -> Object,
    {
        //self.collect_garbage();

        let gc_data = Box::new(GcData {
            header: GcHeader {
                marked: false,
                next: self.permanent_head.take(),
            },
            data,
        });

        let gc = Gc {
            ptr: NonNull::new(Box::into_raw(gc_data)).unwrap(),
        };

        let object = map(gc);

        self.permanent_head = Some(object);

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
            Object::Inst(ptr) => {
                let raw = ptr.ptr.as_ptr();
                drop(Box::from_raw(raw));
            }
        }
    }

    fn drop_object_list(&mut self, head: Option<Object>) {
        let mut current = head;

        while let Some(object) = current {
            let next = match object {
                Object::Str(ref ptr) => ptr.header.next,
                Object::Func(ref ptr) => ptr.header.next,
                Object::Native(ref ptr) => ptr.header.next,
                Object::Arr(ref ptr) => ptr.header.next,
                Object::Inst(ref ptr) => ptr.header.next,
            };

            unsafe {
                self.dealloc(object);
            }

            current = next;
        }
    }
}
impl Drop for Heap {
    fn drop(&mut self) {
        self.drop_object_list(self.head);
        self.drop_object_list(self.permanent_head);
    }
}
