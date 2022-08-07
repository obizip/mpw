use crate::gen::{get_password, PassKind};
use crate::repl::*;
use clap::{Parser, Subcommand};
use colored::*;
use zxcvbn::zxcvbn;
use PassKind::*;

#[derive(Debug, Parser)]
#[clap(
    name = "mpw",
    version = "v1.0.0",
    about = "A simple CLI password generator"
)]

pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate a password
    Gen {
        /// length of a password (>=6)
        #[clap(default_value = "9")]
        length: i32,

        #[clap(
            short,
            long,
            default_value = "0",
            help = "kind of distribution -d(0|1|2|3)\n0 -> alphabets and numbers\n1 -> only alphabets\n2 -> only numbers\n3 -> alphabets, numbers and some signs\nno inputs ->"
        )]
        dist: u8,

        /// print password-strength information
        #[clap(short, long)]
        info: bool,
    },
    /// Check security level for arg
    Check {
        /// Stuff to evaluate security level
        #[clap(value_parser)]
        password: String,

        /// print password-strength information
        #[clap(short, long)]
        info: bool,
    },
    Ls,
}

fn print_password_level(password: &str, info: bool) {
    let estimate = zxcvbn(password, &[]).unwrap();
    let score = estimate.score();
    let score_with_color = match score {
        0 => "0".red(),
        1 => "1".bright_red(),
        2 => "2".yellow(),
        3 => "3".bright_blue(),
        4 => "4".blue(),
        _ => (score as char).to_string().white(),
    };
    println!("password: {}", password);
    println!("strength: {}", score_with_color);
    if info == true {
        println!("{} ------------> {}", "weak".red(), "strong".blue());
        println!(
            "     {}  {}  {}  {}  {}       ",
            "0".red(),
            "1".bright_red(),
            "2".yellow(),
            "3".bright_blue(),
            "4".blue()
        );
        print!("     ");
        for i in 0..=4 {
            if i == score {
                print!("^");
            } else {
                print!("   ");
            }
        }
        println!("");
        match score {
            0 => println!("{} # too guessable: risky password. (guesses < 10^3)", "0".red()),
            1 => println!("{} # very guessable: protection from throttled online attacks. (guesses < 10^6)", "1".bright_red()),
            2 => println!("{} # somewhat guessable: protection from unthrottled online attacks. (guesses < 10^8)", "2".yellow()),
            3 => println!("{} # safely unguessable: moderate protection from offline slow-hash scenario. (guesses < 10^10)","3".bright_blue()),
            4 => println!("{} # very unguessable: strong protection from offline slow-hash scenario. (guesses >= 10^10)", "4".blue()),
            _ => {}
        }
        println!("\nScored by dropbox/zxcvbn (https://github.com/shssoichiro/zxcvbn-rs)");
    }
}

pub fn do_command(args: Cli) {
    match args.command {
        Commands::Gen { length, dist, info } => {
            if length < 5 {
                panic!("length is too short! [length >= 6]");
            }
            let kind = match dist {
                0 => Alphanum,
                1 => Alphabets,
                2 => Numbers,
                3 => All,
                _ => {
                    panic!("Unexpected option! [-d(0|1|2|3) default->0]");
                }
            };
            let password = get_password(kind, length);
            print_password_level(&password, info);
        }
        Commands::Check { password, info } => {
            print_password_level(&password, info);
        }
        Commands::Ls => do_repl(),
    }
}
