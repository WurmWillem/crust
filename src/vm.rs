use colored::Colorize;

use crate::error::DEBUG_TRACE_EXECUTION;
use crate::object::{Object, ObjectValue};
use crate::{chunk::Chunk, opcode::OpCode, value::StackValue};

pub enum InterpretResult {
    Ok,
    // RuntimeError,
}

const STACK_SIZE: usize = 256;

pub struct VM {
    chunk: Chunk,
    ip: *const u8,
    stack: [StackValue; STACK_SIZE],
    stack_top: usize,
    objects: Vec<Object>,
}
impl VM {
    pub fn interpret(chunk: Chunk, objects: Vec<Object>) -> InterpretResult {
        let ip = chunk.get_ptr();
        let mut vm = Self {
            chunk,
            objects,
            ip,
            stack: [const { StackValue::Null }; STACK_SIZE],
            stack_top: 0,
        };
        unsafe { vm.run() }
    }

    fn stack_push(&mut self, value: StackValue) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn stack_pop(&mut self) -> StackValue {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn stack_peek(&mut self) -> StackValue {
        self.stack[self.stack_top - 1]
    }

    #[inline(always)]
    unsafe fn read_byte(&mut self) -> u8 {
        let byte = *self.ip;
        self.ip = self.ip.add(1);
        byte
    }

    #[inline(always)]
    unsafe fn read_short(&mut self) -> u16 {
        self.ip = self.ip.add(2);
        let high = *self.ip.offset(-2);
        let low = *self.ip.offset(-1);
        ((high as u16) << 8) | (low as u16)
    }

    unsafe fn run(&mut self) -> InterpretResult {
        // consider making ip a local variable
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("          ");
                for stack_index in 0..self.stack_top {
                    print!("[ {} ]", self.stack[stack_index].display(&self.objects))
                }
                println!();

                let debug_offset = self.ip.offset_from(self.chunk.code.as_ptr());
                self.chunk
                    .disassemble_instruction(debug_offset as usize, &self.objects);
            }

            macro_rules! binary_op {
                ($operation: ident) => {{
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(lhs.$operation(rhs));
                }};
            }
            // macro_rules! get_var_name_from_next_byte {
            //     () => {{
            //         let constants_index = self.read_byte() as usize;
            //         let StackValue::Obj(idx) = self.chunk.constants[constants_index] else {
            //             unreachable!();
            //         };
            //         let ObjectValue::Str(var_name) = &self.objects[idx].value;
            //         var_name
            //     }};
            // }

            match std::mem::transmute::<u8, OpCode>(self.read_byte()) {
                OpCode::Return => {
                    return InterpretResult::Ok;
                }
                OpCode::Constant => {
                    let index = self.read_byte() as usize;
                    let constant = self.chunk.constants[index];
                    self.stack_push(constant);
                }
                OpCode::Pop => {
                    // dbg!(self.stack_top);
                    self.stack_pop();
                }

                OpCode::Jump => {
                    let offset = self.read_short() as usize;
                    self.ip = self.ip.add(offset);
                }
                OpCode::JumpIfFalse => {
                    let offset = self.read_short() as usize;
                    if let StackValue::Bool(false) = self.stack_peek() {
                        self.ip = self.ip.add(offset);
                    }
                }
                OpCode::Loop => {
                    let offset = self.read_short() as usize;
                    self.ip = self.ip.sub(offset);
                }

                OpCode::Print => {
                    let string = self.stack_pop().display(&self.objects).green();
                    println!("{}", string);
                }

                OpCode::GetLocal => {
                    let slot = self.read_byte();
                    self.stack_push(self.stack[slot as usize]);
                }
                OpCode::SetLocal => {
                    let slot = self.read_byte();
                    self.stack[slot as usize] = self.stack_peek();
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
        }
        // InterpretResult::RuntimeError
    }

    fn concatenate_strings(&mut self, lhs: usize, rhs: usize) -> StackValue {
        // remove rhs so we can take ownership, but mutate lhs so we don't
        // have to remove and then push again
        assert_ne!(lhs, rhs, "lhs and rhs must not be the same object index");

        let lhs_index = lhs;
        // let rhs_value = self.objects.swap_remove(rhs).value;
        let rhs_value = self.objects[rhs].value.clone();
        // let ObjectValue::Str(rhs_value) = self.objects[rhs].value;
        let lhs_value = &mut self.objects[lhs].value;

        let (ObjectValue::Str(lhs), ObjectValue::Str(rhs)) = (lhs_value, rhs_value);

        lhs.push_str(&rhs);
        StackValue::Obj(lhs_index)
    }
}
