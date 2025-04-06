use crate::error::DEBUG_TRACE_EXECUTION;
use crate::{chunk::Chunk, opcode::OpCode, value::StackValue};

pub enum InterpretResult {
    Ok,
    // CompileError,
    // RuntimeError,
}

const STACK_SIZE: usize = 256;

pub struct VM {
    chunk: Chunk,
    ip: *const u8,
    stack: [StackValue; STACK_SIZE],
    stack_top: usize,
}
impl VM {
    pub fn interpret(chunk: Chunk) -> InterpretResult {
        let ip = chunk.get_ptr();
        let mut vm = Self {
            chunk,
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
                    print!("[ {} ]", self.stack[stack_index])
                }
                println!();

                let debug_offset = self.ip.offset_from(self.chunk.code.as_ptr());
                self.chunk.disassemble_instruction(debug_offset as usize);
            }

            macro_rules! binary_op {
                ($operation: ident) => {{
                    let rhs = self.stack_pop();
                    let lhs = self.stack_pop();
                    self.stack_push(lhs.$operation(rhs));
                }};
            }
            match std::mem::transmute::<u8, OpCode>(self.read_byte()) {
                OpCode::Return => {
                    println!("{}", self.stack_pop());
                    return InterpretResult::Ok;
                }
                OpCode::Constant => {
                    let index = self.read_byte() as usize;
                    let constant = self.chunk.constants[index];
                    self.stack_push(constant);
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

                OpCode::Add => binary_op!(add_nums),
                OpCode::Sub => binary_op!(sub_nums),
                OpCode::Mul => binary_op!(mul_nums),
                OpCode::Div => binary_op!(div_nums),
                OpCode::Equal => binary_op!(equals),
                OpCode::Greater => binary_op!(is_greater_than),
                OpCode::GreaterEqual => binary_op!(is_greater_equal_than),
                OpCode::Less => binary_op!(is_less_than),
                OpCode::LessEqual => binary_op!(is_less_equal_than),
            }
        }
        // dbg!(5);
        // InterpretResult::RuntimeError
    }
}
