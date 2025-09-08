use std::fmt::Write;

pub type Value = f64;

#[derive(Debug)]
pub enum OpCode {
    Constant(usize),
    Return,
}

pub type LineNum = u32;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    lines: Vec<(LineNum, u16)>, // RLE, u16 is repetitions
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn push_opcode(&mut self, opcode: OpCode, line: LineNum) {
        self.code.push(opcode);
        self.push_line(line);
    }

    pub fn push_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn push_const_opcode(&mut self, value: Value, line: LineNum) {
        let i = self.push_constant(value);
        self.push_opcode(OpCode::Constant(i), line);
    }

    pub fn push_line(&mut self, line: LineNum) {
        if let Some(last) = self.lines.last_mut()
            && last.0 == line
        {
            last.1 += 1;
            return;
        }

        self.lines.push((line, 1));
    }

    pub fn get_line(&self, mut i: usize) -> Option<LineNum> {
        for (line, count) in self.lines.iter().copied() {
            let count = count as usize;

            if i < count {
                return Some(line);
            }

            i -= count;
        }

        None
    }

    pub fn disassemble_instruction(&self, i: usize) -> Option<String> {
        let line = self.get_line(i)?;

        let mut res = String::new();

        // `write!`ing into a String is infallible
        write!(res, "{:04} {:4} ", i, line).unwrap();

        match self.code.get(i)? {
            OpCode::Constant(const_i) => {
                let constant = self.constants[*const_i];
                writeln!(res, "Constant {const_i}: {}", constant).unwrap();
            }

            OpCode::Return => writeln!(res, "Return").unwrap(),
        };

        Some(res)
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.code.len() {
            write!(f, "{}", self.disassemble_instruction(i).unwrap())?;
        }

        Ok(())
    }
}
