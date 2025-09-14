mod scanner;
mod token;

use crate::InterpretResult;
use crate::chunk::Chunk;

use scanner::Scanner;

pub fn compile(source: &str) -> InterpretResult<Chunk> {
    let c = Chunk::new();

    let scanner = Scanner::new(source);

    for token_res in scanner {
        let token = match token_res {
            Ok(t) => t,
            Err(e) => {
                // TODO: Error handling
                eprintln!("Error: {e:?}");
                continue;
            }
        };
    }

    Ok(c)
}
