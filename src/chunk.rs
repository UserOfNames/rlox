#[derive(Debug)]
pub enum OpCode {
    Return,
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
        }
    }

    pub fn write(&mut self, opcode: OpCode) {
        self.code.push(opcode);
    }
}
