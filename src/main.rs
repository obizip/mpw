pub mod gen;
pub mod repl;
use clap::Parser;
mod cli;
use cli::*;

fn main() {
    let args = Cli::parse();
    do_command(args);
}
