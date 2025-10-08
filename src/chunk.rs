use std::fmt::Write;

use crate::USIZE_SIZE;

pub type Value = f64;

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Add,
    Sub,
    Mul,
    Div,
    Negate,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == OpCode::Constant as u8 => Ok(OpCode::Constant),
            x if x == OpCode::Add as u8 => Ok(OpCode::Add),
            x if x == OpCode::Sub as u8 => Ok(OpCode::Sub),
            x if x == OpCode::Mul as u8 => Ok(OpCode::Mul),
            x if x == OpCode::Div as u8 => Ok(OpCode::Div),
            x if x == OpCode::Negate as u8 => Ok(OpCode::Negate),
            x if x == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err("Invalid opcode"),
        }
    }
}

pub type LineNum = u32;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
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
        self.code.push(opcode as u8);
        self.push_line(line);
    }

    pub fn push_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    /// Called when `self.ip` is pointing to the offset stored immediately after an
    /// `OpCode::Constant`. Resolves it into an offset, fetches the corresponding constant, and
    /// returns (offset, constant).
    pub fn get_constant(&self, lower: usize) -> Option<(usize, Value)> {
        let upper = lower.checked_add(USIZE_SIZE)?;

        let bytes = self.code.get(lower..upper)?;
        // This should be safe since get() already did the bounds checking for us
        let bytes: [u8; USIZE_SIZE] = bytes.try_into().expect("Slice length OOB");

        let const_i = usize::from_ne_bytes(bytes);
        let constant = self.constants.get(const_i).copied()?;

        Some((const_i, constant))
    }

    pub fn push_const_opcode(&mut self, value: Value, line: LineNum) {
        let i = self.push_constant(value);
        self.push_opcode(OpCode::Constant, line);
        for _ in 0..USIZE_SIZE {
            self.push_line(line)
        }
        self.code.extend(i.to_ne_bytes());
    }

    pub fn push_line(&mut self, line: LineNum) {
        // TODO: Push constant offset count
        if let Some(last) = self.lines.last_mut()
            && last.0 == line
        {
            last.1 += 1;
        } else {
            self.lines.push((line, 1));
        }
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
        let instruction = *self.code.get(i)?;
        let instruction: OpCode = instruction.try_into().ok()?;
        match instruction {
            OpCode::Constant => {
                let (const_i, constant) = self.get_constant(i + 1)?;
                writeln!(res, "Constant {const_i}: {}", constant).unwrap();
            }

            OpCode::Negate => writeln!(res, "Negate").unwrap(),
            OpCode::Add => writeln!(res, "Add").unwrap(),
            OpCode::Sub => writeln!(res, "Sub").unwrap(),
            OpCode::Mul => writeln!(res, "Mul").unwrap(),
            OpCode::Div => writeln!(res, "Div").unwrap(),

            OpCode::Return => writeln!(res, "Return").unwrap(),
        };

        Some(res)
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.code.len() {
            let formatted_instruction = self.disassemble_instruction(i).unwrap();
            write!(f, "{}", formatted_instruction)?;
        }

        Ok(())
    }
}
