use crate::chunk::Chunk;
use crate::InterpretResult;

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

    pub fn interpret(chunk: Chunk) -> InterpretResult {
        todo!();
    }
}
