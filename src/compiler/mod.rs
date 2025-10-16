mod parser;
mod scanner;
mod token;

use crate::chunk::Chunk;
use crate::{InterpretError, InterpretResult};

use parser::Parser;
use scanner::Scanner;

pub fn compile(source: &str) -> InterpretResult<Chunk> {
    let c = Chunk::new();

    let scanner = Scanner::new(source);
    let mut parser = Parser::new(scanner);

    // for token_res in scanner {
    //     let token = match token_res {
    //         Ok(t) => t,
    //         Err(e) => {
    //             parser.erred = true;
    //             eprintln!("[Line {}] Syntax error: {}\n", e.line(), e);
    //             parser.panicking = true;
    //             continue;
    //         }
    //     };
    //     parser.panicking = false;
    // }
    //
    if parser.erred {
        Err(InterpretError::Compiler)
    } else {
        Ok(c)
    }
}
