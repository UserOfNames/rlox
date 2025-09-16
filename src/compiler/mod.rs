mod scanner;
mod token;

use thiserror::Error;

use crate::InterpretResult;
use crate::chunk::{Chunk, LineNum};

use scanner::Scanner;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Invalid character '{c}' on line {line}")]
    BadChar { line: LineNum, c: char },
    #[error("Could not parse number literal '{n}' on line {line}")]
    BadNumber { line: LineNum, n: String },
    #[error("Unterminated string on line {line}")]
    UnterminatedString { line: LineNum },
}

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
