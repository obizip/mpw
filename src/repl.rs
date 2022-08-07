use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use ReplCmd::*;

#[derive(Debug)]
enum ReplCmd {
    Add,
    Rm,
    Cd,
    Mkdir,
    Help,
    Quit,
}

fn match_repl_cmd(str: &str) -> ReplCmd {
    match str {
        "add" => Add,
        "rm" => Rm,
        "cd" => Cd,
        "mkdir" => Mkdir,
        "help" => Help,
        "quit" => Quit,
        _ => Help,
    }
}

fn add(curr_path: PathBuf, name: &str) -> PathBuf {
    todo!()
}
fn rm(curr_path: PathBuf, name: &str) -> PathBuf {
    todo!()
}
fn mkdir(curr_path: PathBuf, name: &str) -> PathBuf {
    let mut path = curr_path.clone();
    let ret_path = curr_path;
    path.push(name.to_string());
    fs::create_dir(path).expect("can't create_dir");

    return ret_path;
}
fn cd(curr_path: PathBuf, path: &str) -> PathBuf {
    todo!()
}
fn print_help() {
    todo!()
}

fn get_curr_dir() -> PathBuf {
    let path = env::current_dir().expect("can't get curr_dir");
    println!("starting dir: {}", path.display());
    return path;
}

pub fn do_repl() {
    println!("This is mpw repl");
    let mut curr_path = get_curr_dir();
    let mut quit_flag = 0;
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        println!("line is {}", &line);

        let args: Vec<&str> = line.trim_end().split_whitespace().collect();
        let cmd = match_repl_cmd(args[0]);
        println!("cmd is {:?}", cmd);

        match args.len() {
            2 => match cmd {
                Add => curr_path = add(curr_path, args[1]),
                Rm => curr_path = rm(curr_path, args[1]),
                Mkdir => curr_path = mkdir(curr_path, args[1]),
                Cd => curr_path = cd(curr_path, args[1]),
                _ => println!("incorrect input"),
            },
            1 => match cmd {
                Help => print_help(),
                Quit => quit_flag = 1,
                _ => println!("incorrect input"),
            },
            _ => println!("incorrect input!"),
        }

        if quit_flag == 1 {
            break;
        }
    }
}
