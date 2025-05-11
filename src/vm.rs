use std::mem::MaybeUninit;

use crate::object::{Gc, Heap, ObjFunc};
use colored::Colorize;

use crate::error::DEBUG_TRACE_EXECUTION;
use crate::object::Object;
use crate::{opcode::OpCode, value::StackValue};

pub enum InterpretResult {
    Ok,
    // RuntimeError,
}

const STACK_SIZE: usize = 256;
const FRAMES_SIZE: usize = 64;

#[derive(Debug)]
#[repr(C)]
struct CallFrame {
    func: Gc<ObjFunc>,
    ip: *const u8,
    slots: usize,
}

pub struct VM {
    frames: [MaybeUninit<CallFrame>; FRAMES_SIZE],
    frame_count: usize,
    stack: [StackValue; STACK_SIZE],
    stack_top: usize,
    heap: Heap,
}
impl VM {
    pub fn interpret(func: ObjFunc, mut heap: Heap) -> InterpretResult {
        let (func_object, gc_obj) = heap.alloc(func, Object::Func);
        // let x = &gc_obj.data;

        let frames: [MaybeUninit<CallFrame>; FRAMES_SIZE];
        unsafe {
            frames = MaybeUninit::uninit().assume_init();
        }

        let mut vm = Self {
            heap,
            frames,
            frame_count: 1,
            stack: [const { StackValue::Null }; STACK_SIZE],
            stack_top: 0,
        };

        let frame = CallFrame {
            ip: gc_obj.data.chunk.get_ptr(),
            slots: 0,
            func: gc_obj,
        };

        unsafe { vm.frames[0].as_mut_ptr().write(frame) }

        vm.stack_push(StackValue::Obj(func_object));

        unsafe { vm.run() }
        // InterpretResult::Ok
    }

    #[inline(always)]
    fn stack_push(&mut self, value: StackValue) {
        unsafe {
            let top = self.stack.as_mut_ptr().add(self.stack_top);
            top.write(value);
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

    unsafe fn run(&mut self) -> InterpretResult {
        let mut frame = self.frames[self.frame_count - 1].as_mut_ptr();

        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("          ");
                for stack_index in 0..self.stack_top {
                    print!("[ {} ]", self.stack[stack_index].display())
                }
                println!();
                // todo!()

                let ip = (*frame).ip;
                let offset = (*frame).func.data.chunk.code.as_ptr();
                let debug_offset = ip.offset_from(offset) as usize;

                (*frame)
                    .func
                    .data
                    .chunk
                    .disassemble_instruction(debug_offset);
            }

            macro_rules! binary_op {
                ($operation: ident) => {{
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(lhs.$operation(rhs));
                }};
            }

            let opcode = std::mem::transmute::<u8, OpCode>(self.read_byte(frame));
            match opcode {
                OpCode::Return => {
                    let result = self.stack_pop();

                    self.frame_count -= 1;
                    if self.frame_count == 0 {
                        self.stack_pop();
                        return InterpretResult::Ok;
                    }

                    self.stack_top = (*frame).slots;
                    self.stack_push(result);
                    // frame = self.frames[self.frame_count - 1].assume_init_mut();
                    // self.stack_top
                }
                OpCode::Constant => {
                    let index = self.read_byte(frame) as usize;
                    let constant = (*frame).func.data.chunk.constants[index];
                    self.stack_push(constant);
                }
                OpCode::Pop => {
                    // dbg!((*frame).ip);
                    self.stack_pop();
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

                OpCode::Print => {
                    let string = self.stack_pop().display().green();
                    println!("{}", string);
                }

                OpCode::Call => {
                    let arg_count = self.read_byte(frame) as usize;
                    // dbg!(arg_count);
                    let value = &self.stack[self.stack_top - arg_count - 1];

                    if let StackValue::Obj(value) = value {
                        if let Object::Func(func) = value {
                            let func = *func;
                            let slots = self.stack_top - arg_count - 1;

                            let frame = CallFrame {
                                ip: func.data.chunk.get_ptr(),
                                slots,
                                func,
                            };

                            unsafe { self.frames[self.frame_count].as_mut_ptr().write(frame) }
                            self.frame_count += 1;
                        } else {
                            unreachable!()
                        }
                    } else {
                        unreachable!()
                    }
                }

                OpCode::GetLocal => {
                    let slot = self.read_byte(frame) as usize;
                    let value = self.stack[(*frame).slots + slot];
                    // dbg!(slot);
                    self.stack_push(value);
                }
                OpCode::SetLocal => {
                    let slot = self.read_byte(frame) as usize;
                    // let value = (*frame).slots.wrapping_add(slot);
                    self.stack[(*frame).slots + slot] = self.stack_peek();
                    // *value = self.stack_peek();
                    // self.stack[slot as usize] = self.stack_peek();
                }

                OpCode::GetFunc => {
                    let slot = self.read_byte(frame) as usize;
                    let value = self.stack[slot];
                    self.stack_push(value);
                }

                OpCode::True => {
                    self.stack_push(StackValue::Bool(true));
                }
                OpCode::False => {
                    self.stack_push(StackValue::Bool(false));
                }
                OpCode::Null => {
                    self.stack_push(StackValue::Null);
                }

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
                OpCode::Equal => {
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(StackValue::Bool(lhs.equals(rhs)));
                }
                OpCode::BangEqual => {
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(StackValue::Bool(!lhs.equals(rhs)));
                }
                OpCode::Greater => binary_op!(is_greater_than),
                OpCode::GreaterEqual => binary_op!(is_greater_equal_than),
                OpCode::Less => binary_op!(is_less_than),
                OpCode::LessEqual => binary_op!(is_less_equal_than),
            }
            frame = self.frames[self.frame_count - 1].assume_init_mut();
            // break InterpretResult::Ok;
        }
        // InterpretResult::RuntimeError
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
