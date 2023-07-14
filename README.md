# Todo List CLI Application

[点击查看中文文档](README_zh.md)

This is a simple command-line interface (CLI) application for managing a todo list. The application is written in Rust and allows users to add, complete, and list tasks.

## Features

- Add a new task to your todo list
- Complete a task on your todo list
- List all tasks on your todo list

## Installation

To install this application, you need to have Rust installed on your machine. If you don't have Rust installed, you can install it via [rustup](https://rustup.rs/).

Clone the repository:

```bash
git clone https://github.com/yourusername/todolist.git
cd todolist
```

Then, build the application using `cargo`:

```bash
cargo build --release
```

The executable will be in the `./target/release/` directory.

## Usage

### Add a task

```bash
cargo run -- add "Buy milk"
```

The application will confirm that the task has been added:

```bash
Added task 1: Buy milk
```

### Complete a task

```bash
cargo run -- complete 1
```

The application will confirm that the task has been completed:

```bash
Completed task 1: Buy milk
```

### List all tasks

```bash
cargo run -- list
```

The application will list all tasks:

```bash
1: Buy milk
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.