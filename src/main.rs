mod executor;
mod parser;

use std::io::{self, Write};

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

fn print_prompt() {
    print!("mysh> ");
    io::stdout().flush().unwrap();
}

fn read_line() -> Option<String> {
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(0) => None,
        Ok(_) => Some(buffer.trim().to_string()),
        Err(_) => panic!("Error in read_line"),
    }
}

fn main() {
    loop {
        print_prompt();
        if let Some(input) = read_line() {
            
            let cmd = parser::parse_input(input);

            executor::execute(cmd);
            
        } else {

            break;
            
        }
    }
    println!("\nBye!");
}
