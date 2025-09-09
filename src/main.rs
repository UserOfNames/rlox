mod chunk;
mod vm;

use std::path::PathBuf;

use clap::Parser;

use vm::VM;

#[derive(Debug, Parser)]
#[command(name = "rlox", author = "UserOfNames", version, about)]
struct Args {
    #[arg(help = "Path to the file to interpret")]
    path: Option<PathBuf>,
}

// TODO: impl Error
#[derive(Debug)]
enum InterpretError {
    CompilerError,
    RuntimeError,
}

type InterpretResult = Result<(), InterpretError>;

fn repl() {
    todo!();
}

fn run_file(p: PathBuf) -> InterpretResult {
    todo!();
}

fn main() -> InterpretResult {
    let args = Args::parse();

    let mut vm = VM::new();

    // match args.path {
    //     Some(p) => run_file(p)?,
    //     None => repl(),
    // }

    use chunk::{Chunk, LineNum, OpCode, Value};
    let mut c = Chunk::new();
    c.push_const_opcode(1.2, 1);
    c.push_const_opcode(3.4, 1);
    c.push_opcode(OpCode::Add, 1);
    c.push_const_opcode(5.6, 1);
    c.push_opcode(OpCode::Negate, 1);
    c.push_opcode(OpCode::Div, 1);
    c.push_opcode(OpCode::Return, 2);
    vm.interpret(c).unwrap();

    Ok(())
}
