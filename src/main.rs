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

    match args.path {
        Some(p) => run_file(p)?,
        None => repl(),
    }

    Ok(())
}
