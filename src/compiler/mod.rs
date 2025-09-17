mod scanner;
mod token;

use crate::chunk::Chunk;
use crate::{InterpretError, InterpretResult};

use scanner::Scanner;

pub fn compile(source: &str) -> InterpretResult<Chunk> {
    let c = Chunk::new();

    let mut erred = false;
    let scanner = Scanner::new(source);

    for token_res in scanner {
        let token = match token_res {
            Ok(t) => t,
            Err(e) => {
                erred = true;
                let line = e.line();
                eprintln!("[Line {line}] Error: {e}");
                continue;
            }
        };
    }

    if erred {
        Err(InterpretError::Compiler)
    } else {
        Ok(c)
    }
}
