mod chunk;
mod compiler; // TODO: Move?
mod vm;

use std::fs::read_to_string;
use std::io::{self, BufRead, Write, stdin, stdout};
use std::path::PathBuf;

use clap::Parser;
use thiserror::Error;

use vm::VM;

#[derive(Debug, Parser)]
#[command(name = "rlox", author = "UserOfNames", version, about)]
struct Args {
    #[arg(help = "Path to the file to interpret")]
    path: Option<PathBuf>,
}

#[derive(Debug, Error)]
pub enum InterpretError {
    #[error("Compiler error")]
    Compiler,
    #[error("Runtime error")]
    Runtime,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

type InterpretResult<T> = Result<T, InterpretError>;

fn repl() {
    let mut vm = VM::new();

    let mut input = String::new();
    let mut stdin = stdin().lock();

    loop {
        print!(":> ");
        stdout().flush().expect("Could not flush stdout");
        input.clear();

        let read_result = stdin.read_line(&mut input);
        if let Err(e) = read_result {
            eprintln!("Error reading input: {e}");
            continue;
        }

        let interpret_result = vm.interpret(&std::mem::take(&mut input));
        if let Err(e) = interpret_result {
            eprintln!("Error interpreting input: {e}");
        }
    }
}

fn run_file(p: PathBuf) -> InterpretResult<()> {
    let mut vm = VM::new();

    let source = read_to_string(p)?;

    vm.interpret(&source)
}

fn main() -> InterpretResult<()> {
    let args = Args::parse();

    match args.path {
        Some(p) => {
            if let Err(e) = run_file(p) {
                eprintln!("{e}");
            }
        }

        None => repl(),
    }

    Ok(())
}
