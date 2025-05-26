use std::mem::MaybeUninit;

use crate::object::{Gc, Heap, ObjFunc};
use colored::Colorize;

use crate::error::DEBUG_TRACE_EXECUTION;
use crate::object::Object;
use crate::{op_code::OpCode, value::StackValue};

pub enum InterpretResult {
    Ok,
    // RuntimeError,
}

const STACK_SIZE: usize = 256;
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
        let (func_object, gc_obj) = heap.alloc(func, Object::Func);

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
            stack: [const { StackValue::Null }; STACK_SIZE],
            stack_top: 0,
        };

        vm.stack_push(StackValue::Obj(func_object));

        unsafe { vm.run() }
    }

    unsafe fn run(&mut self) -> InterpretResult {
        let mut frame = self.frames.as_mut_ptr().add(self.frame_count - 1);

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

            let op_code = std::mem::transmute::<u8, OpCode>(self.read_byte(frame));
            match op_code {
                OpCode::Pop => {
                    self.pop_no_return();
                }
                OpCode::Constant => {
                    let index = self.read_byte(frame) as usize;
                    let constant = (*frame).func.data.chunk.constants[index];
                    self.stack_push(constant);
                }

                OpCode::GetLocal => {
                    let slot = self.read_byte(frame) as usize;
                    let value = self.stack[(*frame).slots + slot];
                    self.stack_push(value);
                }
                OpCode::SetLocal => {
                    let slot = self.read_byte(frame) as usize;
                    self.stack[(*frame).slots + slot] = self.stack_peek();
                }

                OpCode::Call => {
                    self.call(frame);
                    frame = self.frames.as_mut_ptr().add(self.frame_count - 1);
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
                }

                OpCode::Jump => {
                    let offset = self.read_short(frame) as usize;
                    (*frame).ip = (*frame).ip.add(offset);
                }
                OpCode::JumpIfFalse => {
                    let offset = self.read_short(frame) as usize;
                    if let StackValue::Bool(false) = self.stack_peek() {
                        (*frame).ip = (*frame).ip.add(offset);
                    }
                }
                OpCode::Loop => {
                    let offset = self.read_short(frame) as usize;
                    (*frame).ip = (*frame).ip.sub(offset);
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
                    let string = self.stack_pop().to_string().green();
                    println!("{}", string);
                }
            }
        }
    }

    unsafe fn call(&mut self, frame: *mut CallFrame) {
        let arg_count = self.read_byte(frame) as usize;
        let slots = self.stack_top - arg_count;
        let value = self.stack[slots];

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

                    let value = (func.data.func)(args);

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

        let (object, _) = self.heap.alloc(new_str, Object::Str);

        StackValue::Obj(object)
    }
}
