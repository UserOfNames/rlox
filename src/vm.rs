use crate::chunk::{Chunk, OpCode, Value};
use crate::compiler::compile;
use crate::{InterpretError, InterpretResult};

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult<()> {
        self.chunk = compile(source)?;
        self.run()
    }

    fn run(&mut self) -> InterpretResult<()> {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                println!("{:?}", self.stack);
                println!("{}", self.chunk.disassemble_instruction(self.ip).unwrap());
            }

            let instruction = &self
                .chunk
                .code
                .get(self.ip)
                .ok_or(InterpretError::Runtime)?;
            self.ip += 1;

            match instruction {
                OpCode::Constant(c) => {
                    // TODO: Model this properly (if Value isn't copy, deref will move)
                    let constant = *self
                        .chunk
                        .constants
                        .get(*c)
                        .ok_or(InterpretError::Runtime)?;
                    self.stack.push(constant);
                }

                OpCode::Negate => {
                    // TODO: Is expect appropriate here?
                    let operand = self
                        .stack
                        .pop()
                        .expect("Attempted to pop value, but the stack was empty");
                    self.stack.push(-operand);
                }

                OpCode::Add => self.binary_operator(std::ops::Add::add),
                OpCode::Sub => self.binary_operator(std::ops::Sub::sub),
                OpCode::Mul => self.binary_operator(std::ops::Mul::mul),
                OpCode::Div => self.binary_operator(std::ops::Div::div),

                OpCode::Return => {
                    println!("Returned: {:?}", self.stack.pop());
                    return Ok(());
                }
            }
        }
    }

    // TODO: This helper will probably not work when Value becomes more complex
    fn binary_operator<F>(&mut self, operator: F)
    where
        F: Fn(Value, Value) -> Value,
    {
        let r = self
            .stack
            .pop()
            .expect("Attempted to pop value, but the stack was empty");
        let l = self
            .stack
            .pop()
            .expect("Attempted to pop value, but the stack was empty");
        self.stack.push(operator(l, r));
    }
}
