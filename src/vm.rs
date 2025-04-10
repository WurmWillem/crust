use std::collections::HashMap;

use colored::Colorize;

use crate::error::{print_error, DEBUG_TRACE_EXECUTION};
use crate::object::{Object, ObjectValue};
use crate::{chunk::Chunk, opcode::OpCode, value::StackValue};

pub enum InterpretResult {
    Ok,
    RuntimeError,
}

const STACK_SIZE: usize = 256;

pub struct VM {
    chunk: Chunk,
    ip: *const u8,
    stack: [StackValue; STACK_SIZE],
    stack_top: usize,
    objects: Vec<Object>,
    globals: HashMap<String, StackValue>,
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
            globals: HashMap::new(),
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
            macro_rules! get_var_name_from_next_byte {
                () => {{
                    let constants_index = self.read_byte() as usize;
                    let StackValue::Obj(idx) = self.chunk.constants[constants_index] else {
                        unreachable!();
                    };
                    let ObjectValue::Str(var_name) = &self.objects[idx].value;
                    var_name
                }};
            }

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

                OpCode::Print => {
                    let string = self.stack_pop().display(&self.objects).green();
                    println!("{}", string);
                }

                OpCode::DefineGlobal => {
                    let value = self.stack_pop();
                    let var_name = get_var_name_from_next_byte!();

                    self.globals.insert(var_name.clone(), value);
                }
                OpCode::GetGlobal => {
                    let var_name = get_var_name_from_next_byte!();
                    let value = match self.globals.get(var_name) {
                        Some(value) => value,
                        _ => {
                            let debug_offset = self.ip.offset_from(self.chunk.code.as_ptr());
                            let line = self.chunk.lines[debug_offset as usize];

                            let msg = format!("The variable with name '{}' does not exist.", var_name);
                            print_error(line, &msg);

                            return InterpretResult::RuntimeError;
                        }
                    };
                    self.stack_push(*value);
                }
                OpCode::SetGlobal => {
                    let new_value = self.stack_peek();
                    let var_name = get_var_name_from_next_byte!();

                    let value = match self.globals.get_mut(var_name) {
                        Some(value) => value,
                        _ => {
                            let debug_offset = self.ip.offset_from(self.chunk.code.as_ptr());
                            let line = self.chunk.lines[debug_offset as usize];

                            let msg = format!("The variable with name '{}' does not exist.", var_name);
                            print_error(line, &msg);

                            return InterpretResult::RuntimeError;
                        }
                    };

                    *value = new_value;
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

        let (ObjectValue::Str(lhs), ObjectValue::Str(rhs)) = (lhs_value, rhs_value) else {
            unreachable!();
        };

        lhs.push_str(&rhs);
        StackValue::Obj(lhs_index)
    }
}
