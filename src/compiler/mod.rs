mod scanner;
mod token;

use std::fmt;

use crate::InterpretResult;
use crate::chunk::{Chunk, LineNum};

use scanner::Scanner;

#[derive(Debug)]
pub enum CompilerError {
    BadChar(LineNum, char),
    BadNumber(LineNum, String),
    UnterminatedString(LineNum),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadChar(line, c) => {
                write!(f, "Invalid character '{c}' on line {line}")
            }

            Self::BadNumber(line, n) => {
                write!(f, "Could not parse number literal '{n}' on line {line}")
            }

            Self::UnterminatedString(line) => {
                write!(f, "Unterminated string on line {line}")
            }
        }
    }
}

impl std::error::Error for CompilerError {}

pub type CompilerResult<T> = Result<T, CompilerError>;

pub fn compile(source: &str) -> InterpretResult<Chunk> {
    let c = Chunk::new();

    let scanner = Scanner::new(source);

    for token_res in scanner {
        let token = match token_res {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };
    }

    Ok(c)
}
