use crate::{chunk::Chunk, opcode::OpCode, value::Value};

const DEBUG_TRACE_EXECUTION: bool = true;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

const STACK_SIZE: usize = 256;

pub struct VM {
    chunk: Chunk,
    ip: *const u8,
    stack: [Value; STACK_SIZE],
    stack_top: usize,
}
impl VM {
    // fn new() -> Self {
    //     let chunk =
    //     Self { chunk: Chunk::new(), ip: (), stack: [Value::None; STACK_SIZE], stack_top: () }
    // }

    pub fn interpret(chunk: Chunk) -> InterpretResult {
        let ip = chunk.get_ptr();
        let mut vm = Self {
            chunk,
            ip,
            stack: [const { Value::None }; STACK_SIZE],
            stack_top: 0,
        };
        // self.chunk = chunk;
        // self.ip = self.chunk.get_ptr();
        unsafe { vm.run() }
    }

    fn stack_push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn stack_pop(&mut self) -> &Value {
        self.stack_top -= 1;
        &self.stack[self.stack_top]
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
            // let instruction = self.ip;
            // let byte = *self.ip;
            // self.ip = self.ip.add(1);

            match std::mem::transmute::<u8, OpCode>(self.read_byte()) {
                OpCode::Return => {
                    println!("{}", self.stack_pop());
                    return InterpretResult::Ok;
                }
                OpCode::Constant => {
                    let index = self.read_byte() as usize;
                    let constant = self.chunk.constants[index].clone();
                    // println!("hey");

                    self.stack_push(constant);
                    // break;
                }
            }
        }
        // dbg!(5);
        // InterpretResult::RuntimeError
    }
}
