mod chunk;

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rlox", author = "UserOfNames", version, about)]
struct Args {
    #[arg(help = "Path to the file to interpret")]
    path: Option<PathBuf>,
}

type InterpretResult<T> = Result<T, ()>;

fn repl() {
    todo!();
}

fn run_file(p: PathBuf) -> InterpretResult<()> {
    todo!();
}

fn main() -> InterpretResult<()> {
    let args = Args::parse();

    // match args.path {
    //     Some(p) => run_file(p)?,
    //     None => repl(),
    // }

    use chunk::{Chunk, Value, LineNum, OpCode};
    let mut c = Chunk::new();
    c.push_const_opcode(4.2, 1);
    c.push_opcode(OpCode::Return, 2);
    println!("{c}");

    Ok(())
}
