use crate::Command;

pub fn parse_input(input: String) -> Command {
    let mut tokens = input.split_ascii_whitespace();

    let name = tokens.next().unwrap_or("").to_string();
    let args = tokens.map(|s| s.to_string()).collect();

    Command {name, args}
}

// pub fn parse_input(input: String) -> Command {
    
//     let mut cmd: Command = Command {
//         name: "".to_string(),
//         args: Vec::new()
//     };

//     for token in input.split_ascii_whitespace() {
//         if cmd.name.is_empty() {
//             cmd.name = token.to_string();
//         } else {
//             cmd.args.push(token.to_string());
//         }
//     }

//     cmd
// }
