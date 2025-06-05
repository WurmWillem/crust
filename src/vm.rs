use colored::Colorize;

use crate::{
    error::DEBUG_TRACE_EXECUTION,
    heap::Heap,
    object::{Gc, ObjArr, ObjFunc, ObjInstance, Object},
    op_code::OpCode,
    value::StackValue,
};

pub enum InterpretResult {
    Ok,
    // RuntimeError,
}

pub const STACK_SIZE: usize = 256;
const FRAMES_SIZE: usize = 64;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct CallFrame {
    func: Gc<ObjFunc>,
    ip: *const u8,
    slots: usize,
}

pub struct VM {
    frames: [CallFrame; FRAMES_SIZE],
    frame_count: usize,
    stack: [StackValue; STACK_SIZE],
    stack_top: usize,
    heap: Heap,
}
impl VM {
    pub fn interpret(func: ObjFunc, mut heap: Heap) -> InterpretResult {
        let (func_object, gc_obj) = heap.alloc_permanent(func, Object::Func);

        let frame = CallFrame {
            ip: gc_obj.data.chunk.get_ptr(),
            slots: 0,
            func: gc_obj,
        };
        let frames = [frame; FRAMES_SIZE];

        let mut vm = Self {
            heap,
            frames,
            frame_count: 1,
            stack: [StackValue::Null; STACK_SIZE],
            stack_top: 0,
        };

        vm.stack_push(StackValue::Obj(func_object));

        unsafe { vm.run() }
    }

    unsafe fn run(&mut self) -> InterpretResult {
        let mut frame = self.frames.as_mut_ptr().add(self.frame_count - 1);
        let mut ip = (*frame).ip;

        loop {
            if DEBUG_TRACE_EXECUTION {
                self.debug_trace(frame)
            }

            macro_rules! binary_op {
                ($operation: ident) => {{
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(lhs.$operation(rhs));
                }};
            }

            let op_code = std::mem::transmute::<u8, OpCode>(self.rb(&mut ip));
            match op_code {
                OpCode::Pop => {
                    self.pop_no_return();
                }
                OpCode::Constant => {
                    let index = self.rb(&mut ip) as usize;
                    let constant = (*frame).func.data.chunk.constants[index];
                    self.stack_push(constant);
                }

                OpCode::GetLocal => {
                    let slot = self.rb(&mut ip) as usize;
                    let value = self.stack[(*frame).slots + slot];
                    self.stack_push(value);
                }
                OpCode::SetLocal => {
                    let slot = self.rb(&mut ip) as usize;
                    self.stack[(*frame).slots + slot] = self.stack_peek();
                }

                OpCode::AllocArr => {
                    let len = self.stack_pop();
                    let len = if let StackValue::F64(len) = len {
                        len as usize
                    } else {
                        unreachable!()
                    };
                    let mut values = Vec::new();
                    for _ in 0..len {
                        values.push(self.stack_pop());
                    }

                    let obj = ObjArr::new(values);
                    let (object, _) =
                        self.heap
                            .alloc(obj, Object::Arr, &mut self.stack, self.stack_top);
                    let arr = StackValue::Obj(object);
                    self.stack_push(arr);
                }
                OpCode::IndexArr => {
                    let StackValue::F64(index) = self.stack_pop() else {
                        unreachable!()
                    };

                    let arr = self.stack_pop();
                    if let StackValue::Obj(Object::Arr(arr)) = arr {
                        let value = arr.data.values[index as usize];
                        self.stack_push(value);
                    }
                }
                OpCode::AssignIndex => {
                    let new_value = self.stack_pop();
                    let StackValue::F64(index) = self.stack_pop() else {
                        unreachable!()
                    };

                    let arr = self.stack_peek();
                    if let StackValue::Obj(Object::Arr(mut arr)) = arr {
                        arr.data.values[index as usize] = new_value;
                    }
                }

                OpCode::FuncCall => {
                    (*frame).ip = ip;
                    self.call(frame);
                    frame = self.frames.as_mut_ptr().add(self.frame_count - 1);
                    ip = (*frame).ip;
                }
                OpCode::MethodCall => {
                    let index = self.rb(&mut ip) as usize;
                    let inst = self.stack_pop();
                    let StackValue::Obj(Object::Instance(inst)) = inst else {
                        unreachable!()
                    };

                    let method = inst.data.methods[index];
                    self.stack_push(method);
                }

                OpCode::AllocInstance => {
                    let methods_len = self.rb(&mut ip) as usize;
                    let fields_len = self.rb(&mut ip) as usize;

                    let mut fields = Vec::new();
                    for _ in 0..fields_len {
                        fields.push(self.stack_pop());
                    }
                    // dbg!(&fields);

                    let mut methods = Vec::new();
                    for _ in 0..methods_len {
                        methods.push(self.stack_pop());
                    }
                    // dbg!(&methods);

                    let inst = ObjInstance::new(fields, methods);
                    let (obj, _) =
                        self.heap
                            .alloc(inst, Object::Instance, &mut self.stack, self.stack_top);
                    let obj = StackValue::Obj(obj);
                    self.stack_push(obj);
                }
                OpCode::GetPubField => {
                    let index = self.rb(&mut ip) as usize;
                    let inst = self.stack_pop();
                    let StackValue::Obj(Object::Instance(inst)) = inst else {
                        unreachable!()
                    };
                    self.stack_push(inst.data.fields[index]);
                }
                OpCode::SetPubField => {
                    let new_value = self.stack_pop();
                    let index = self.rb(&mut ip) as usize;
                    let inst = self.stack_peek();
                    let StackValue::Obj(Object::Instance(mut inst)) = inst else {
                        unreachable!()
                    };
                    inst.data.fields[index] = new_value;
                }
                OpCode::GetSelfField => {
                    let index = self.rb(&mut ip) as usize;
                    let inst = self.stack[(*frame).slots - 1];
                    // dbg!(inst);
                    let StackValue::Obj(Object::Instance(inst)) = inst else {
                        unreachable!()
                    };
                    self.stack_push(inst.data.fields[index]);
                }
                OpCode::GetSetField => {
                    let new_value = self.stack_pop();
                    let index = self.rb(&mut ip) as usize;
                    let inst = self.stack[(*frame).slots - 1];
                    let StackValue::Obj(Object::Instance(mut inst)) = inst else {
                        unreachable!()
                    };
                    inst.data.fields[index] = new_value;
                }

                OpCode::Return => {
                    let result = self.stack_pop();

                    self.frame_count -= 1;
                    if self.frame_count == 0 {
                        self.pop_no_return();
                        return InterpretResult::Ok;
                    }

                    self.stack_top = (*frame).slots;
                    self.stack_push(result);
                    frame = self.frames.as_mut_ptr().add(self.frame_count - 1);
                    ip = (*frame).ip;
                }

                OpCode::Jump => {
                    let offset = self.rs(&mut ip) as usize;
                    ip = ip.add(offset);
                }
                OpCode::JumpIfFalse => {
                    let offset = self.rs(&mut ip) as usize;
                    if let StackValue::Bool(false) = self.stack_peek() {
                        ip = ip.add(offset);
                    }
                }
                OpCode::Loop => {
                    let offset = self.rs(&mut ip) as usize;
                    ip = ip.sub(offset);
                }

                OpCode::True => self.stack_push(StackValue::Bool(true)),
                OpCode::False => self.stack_push(StackValue::Bool(false)),
                OpCode::Null => self.stack_push(StackValue::Null),

                OpCode::Negate => {
                    let new_value = -self.stack_pop();
                    self.stack_push(new_value);
                }
                OpCode::Not => {
                    let new_value = !self.stack_pop();
                    self.stack_push(new_value);
                }

                OpCode::Add => {
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();

                    let new_value = match (lhs, rhs) {
                        (StackValue::F64(lhs), StackValue::F64(rhs)) => StackValue::F64(lhs + rhs),
                        (StackValue::Obj(lhs), StackValue::Obj(rhs)) => {
                            self.concatenate_strings(lhs, rhs)
                        }
                        _ => unreachable!(),
                    };

                    self.stack_push(new_value);
                }
                OpCode::Sub => binary_op!(sub_nums),
                OpCode::Mul => binary_op!(mul_nums),
                OpCode::Div => binary_op!(div_nums),
                OpCode::And => {
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(StackValue::Bool(lhs.and(rhs)));
                }
                OpCode::Or => {
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(StackValue::Bool(lhs.or(rhs)));
                }
                OpCode::Equal => {
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(StackValue::Bool(lhs.equals(rhs)));
                }
                OpCode::NotEqual => {
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(StackValue::Bool(!lhs.equals(rhs)));
                }
                OpCode::Greater => binary_op!(is_greater_than),
                OpCode::GreaterEqual => binary_op!(is_greater_equal_than),
                OpCode::Less => binary_op!(is_less_than),
                OpCode::LessEqual => binary_op!(is_less_equal_than),
                OpCode::Print => {
                    let string = self.stack_pop().display().green();
                    println!("{}", string);
                }
            }
        }
    }

    unsafe fn call(&mut self, frame: *mut CallFrame) {
        let arg_count = self.read_byte(frame) as usize;
        let slots = self.stack_top - arg_count;
        // dbg!(arg_count);
        let value = self.stack[slots];
        // dbg!(self.stack[self.stack_top - 1]);
        // dbg!(value);

        if let StackValue::Obj(obj) = value {
            match obj {
                Object::Func(func) => {
                    let frame = CallFrame {
                        ip: func.data.chunk.get_ptr(),
                        slots,
                        func,
                    };

                    unsafe { self.frames.as_mut_ptr().add(self.frame_count).write(frame) }
                    self.frame_count += 1;
                    //*frameee = self.frames[self.frame_count - 1].assume_init_mut();
                }
                Object::Native(func) => {
                    let args_ptr = self.stack.as_ptr().add(slots + 1);
                    let args = std::slice::from_raw_parts(args_ptr, arg_count);

                    let value = (func.data.func)(args, &mut self.heap);

                    self.stack_top = slots;
                    self.stack_push(value);
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }

    #[inline(always)]
    fn stack_push(&mut self, value: StackValue) {
        unsafe {
            *self.stack.get_unchecked_mut(self.stack_top) = value;
            self.stack_top += 1;
        }
    }

    #[inline(always)]
    fn stack_pop(&mut self) -> StackValue {
        unsafe {
            self.stack_top -= 1;
            self.stack.as_ptr().add(self.stack_top).read()
        }
    }

    #[inline(always)]
    fn pop_no_return(&mut self) {
        self.stack_top -= 1;
    }

    #[inline(always)]
    fn stack_peek(&mut self) -> StackValue {
        unsafe { self.stack.as_ptr().add(self.stack_top - 1).read() }
    }

    #[inline(always)]
    unsafe fn read_byte(&mut self, frame: *mut CallFrame) -> u8 {
        // let mut ip = frame.ip;
        let byte = *(*frame).ip;
        (*frame).ip = (*frame).ip.add(1);
        byte
    }

    #[inline(always)]
    unsafe fn rb(&mut self, ip: &mut *const u8) -> u8 {
        unsafe {
            let byte = **ip;
            *ip = ip.add(1);
            byte
        }
    }

    #[inline(always)]
    unsafe fn rs(&mut self, ip: &mut *const u8) -> u16 {
        *ip = ip.add(2);

        let high = *ip.offset(-2);
        let low = *ip.offset(-1);

        ((high as u16) << 8) | (low as u16)
    }

    #[inline(always)]
    unsafe fn read_short(&mut self, frame: *mut CallFrame) -> u16 {
        let ip = &mut (*frame).ip;
        *ip = ip.add(2);

        let high = *ip.offset(-2);
        let low = *ip.offset(-1);

        ((high as u16) << 8) | (low as u16)
    }

    unsafe fn debug_trace(&self, frame: *mut CallFrame) {
        print!("          ");
        for stack_index in 0..self.stack_top {
            print!("[ {} ]", self.stack[stack_index].display())
        }
        println!();

        let ip = (*frame).ip;
        let offset = (*frame).func.data.chunk.code.as_ptr();
        let debug_offset = ip.offset_from(offset) as usize;

        (*frame)
            .func
            .data
            .chunk
            .disassemble_instruction(debug_offset);
    }

    fn concatenate_strings(&mut self, lhs: Object, rhs: Object) -> StackValue {
        let Object::Str(lhs) = lhs else {
            unreachable!()
        };
        let Object::Str(rhs) = rhs else {
            unreachable!()
        };

        let mut new_str = lhs.data.clone();
        new_str.push_str(&rhs.data);

        let (object, _) = self
            .heap
            .alloc(new_str, Object::Str, &mut self.stack, self.stack_top);

        StackValue::Obj(object)
    }
}
