use std::env;
use std::process;

use crate::Command;

pub fn execute(cmd: Command) {
    match cmd.name.as_str() {
        "cd" => change_directory(cmd.args),
        "pwd" => print_working_directory(),
        "exit" => {
            println!("\nBye!");
            process::exit(0);
        },
        _ => run_external_terminal(cmd),
    }
}

fn change_directory(args: Vec<String>) {
    if let Some(path) = args.get(0) {
        if let Err(e) = env::set_current_dir(path) {
            eprintln!("cd error {e}");
        }
    } else {
        eprintln!("cd: missing argument");
    }
}

fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd error: {}", e),
    }
}

fn run_external_terminal(cmd: Command) {
    let result = process::Command::new(&cmd.name)
        .args(&cmd.args)
        .spawn()
        .and_then(|mut child| child.wait());

    if let Err(e) = result {
        eprintln!("Failed to execute {}: {}", cmd.name, e);
    }
}