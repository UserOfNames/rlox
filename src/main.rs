mod chunk;
mod compiler; // TODO: Move?
mod vm;

use std::fmt;
use std::fs::read_to_string;
use std::io::{self, BufRead, Write, stdin, stdout};
use std::path::PathBuf;

use clap::Parser;

use compiler::CompilerError;
use vm::VM;

#[derive(Debug, Parser)]
#[command(name = "rlox", author = "UserOfNames", version, about)]
struct Args {
    #[arg(help = "Path to the file to interpret")]
    path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum InterpretError {
    Compiler(CompilerError),
    Runtime,
    Io(io::Error),
}

impl From<io::Error> for InterpretError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compiler(_) => write!(f, "Could not compile"),
            Self::Runtime => write!(f, "Runtime error"),
            Self::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for InterpretError {}

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

    let interpret_result = vm.interpret(&source);

    interpret_result
}

fn main() -> InterpretResult<()> {
    let args = Args::parse();

    match args.path {
        Some(p) => run_file(p)?,
        None => repl(),
    }

    Ok(())
}
