use crate::chunk::{Chunk, OpCode};
use crate::{InterpretError, InterpretResult};

pub struct VM {
    chunk: Chunk,
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        let instructions = &self.chunk.code;
        loop {
            #[cfg(feature = "debug_trace_execution")]
            print!("{}", self.chunk.disassemble_instruction(self.ip).unwrap());

            let instruction = instructions
                .get(self.ip)
                .ok_or(InterpretError::RuntimeError)?;
            self.ip += 1;


            match instruction {
                OpCode::Return => return Ok(()),
                OpCode::Constant(c) => {
                    let constant = self
                        .chunk
                        .constants
                        .get(*c)
                        .ok_or(InterpretError::RuntimeError)?;
                }
            }
        }
    }
}
