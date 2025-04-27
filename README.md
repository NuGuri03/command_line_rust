# An I/O Project: Building a Command Line Program

This is a personal project to implement a command-line program based on basic Rust learning.

---

## 🛠 Build and Run

```bash
cargo build
cargo run
```

---

## 📦 First Step

- **Built-in Command Handling**
  - `cd <dir>`: Change directory
  - `pwd`: Print current working directory
  - `exit`: Exit the shell
- **Execute External Commands**
  - Run system commands like `ls`, `date`, etc.
- **Input Parsing**
  - Split input by whitespace to construct an argument array (`Vec<String>`)

---

## 📁 Project Structure

```bash
rusty_shell/
├── src/
│   ├── main.rs       # Main loop
│   ├── parser.rs     # Input parsing module
│   └── executor.rs   # Command execution module
├── Cargo.toml        # Rust package configuration
└── README.md         # Project overview
```

---

## 🔜 Planned Features

- Support for pipes (`|`)
- Input/output redirection (`>`, `<`)
- Background execution (`&`)
- Command history storage
- Support for `.rustyrc` configuration file
- Refactor project structure (modularization, error handling improvement)

---

## 🦠 Rust Technologies Used

- Rust Standard Library
  - `std::env` (handling `cd`, `pwd`)
  - `std::process::Command` (executing external commands)
  - `std::io` (handling input)