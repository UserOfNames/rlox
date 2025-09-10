mod scanner;
mod token;

use crate::InterpretResult;
use crate::chunk::LineNum;

use super::compiler::token::{Token, TokenKind};
use scanner::Scanner;

pub fn compile(source: &str) -> InterpretResult<()> {
    let mut source_iter = source.char_indices();
    let mut scanner = Scanner::new(source);
    for token in scanner {
        let token = token?;
        println!("{token}");
    }

    Ok(())
}
