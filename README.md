# An I/O Project: Building a Command Line Program

This is a personal project to implement a command-line program based on basic Rust learning.

---

## 🛠 Build and Run

```bash
cargo build
cargo run
```

---

## 📦 Features

✅ **Completed Features**
- **Built-in Command Handling**
  - `cd <dir>`: Change directory
  - `pwd`: Print current working directory
  - `exit`: Exit the shell
- **Execute External Commands**
  - Run system commands like `ls`, `date`, etc.
- **Input Parsing**
  - Split input by whitespace to construct an argument array (`Vec<String>`)

🔧 **Planned / In Development**
- **Support for pipes (`|`)**
- **I/O redirection (`>`, `<`)**
- **Background execution (`&`)**
- **Command history storage**
- **Support for `.myshellrc` configuration file**

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


## 🦠 Rust Technologies Used
- Rust Standard Library
  - `std::env` (handling `cd`, `pwd`)
  - `std::process::Command` (executing external commands)
  - `std::io` (handling input)