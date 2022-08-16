pub mod gen;
use clap::Parser;
mod cli;
use cli::*;

fn main() {
    let args = Cli::parse();
    if let Err(e) = do_command(args) {
        println!("Error: {}", e);
    }
}
