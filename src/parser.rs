use crate::Command;

pub fn parse_input(input: String) -> Command {
    let mut tokens = input.split_ascii_whitespace();

    let name = tokens.next().unwrap_or("").to_string();
    let args = tokens.map(|s| s.to_string()).collect();

    Command {name, args}
}