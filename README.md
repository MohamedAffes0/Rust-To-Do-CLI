# 📝 Rust To-Do CLI

A lightweight, interactive command-line To-Do list manager written in Rust. This tool allows you to create, view, update, and delete tasks — all from your terminal — with automatic persistence via a local JSON file.

---

## 🚀 Features

- ✅ Add new tasks with title and description
- 📋 View the entire task list or a specific task
- 🔁 Mark tasks as completed or not completed
- ❌ Delete tasks by ID
- 💾 Persistent data stored in `todo_data.json`
- 🧩 Modular codebase with structured error handling
- 🎛️ Interactive menu using [`inquire`](https://docs.rs/inquire)

---

## 🗂️ Project Structure

```
src/
├── main.rs # Entry point: runs the CLI and handles saving/loading
├── cli.rs # Command parser and executor (CLI interface)
├── to_do.rs # Defines the ToDo struct and its logic
└── to_do_list.rs # Handles list storage, manipulation, and persistence
todo_data.json # JSON file automatically created to store ToDos
```
---

## 📦 Installation & Usage
### Prerequisites
You need to have Rust and Cargo installed on your system.
To install Rust, use [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Build the project
In the project root directory, run:


```bash
cargo build --release
```
This will create an optimized executable in the target/release/ folder.

### Run the application
To run the program in development mode (with auto recompilation):
```
cargo run
```

### 🔧 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### ▶️ Build and Run

```bash
git clone https://github.com/MohamedAffes0/Rust-To-Do-CLI.git
cd Rust-To-Do-CLI
cargo run
```
---
### 📌 How It Works
- Launches an interactive terminal menu

- Select a command from a list of options

- Input is validated; errors are handled gracefully

- All changes are saved to `todo_data.json` after every operation
---
## 🚀Examples
Here are some example interactions to help you get started with the To-Do CLI application.

### Insert a new task
```
Please choose a command:
▸ Insert a new item
Enter the title of the new item:
Buy groceries
Enter the description of the new item:
Milk, eggs, bread
Inserted: Buy groceries [❌]
    Milk, eggs, bread
    Created: 2025-06-13 10:45:21
```
### View all tasks
```
Please choose a command:
▸ Display the entire list
1: Buy groceries [❌]
    Milk, eggs, bread
    Created: 2025-06-13 10:45:21
```
### Mark a task as completed
```
Please choose a command:
▸ Modify the state of an item
Enter the ID of the item to modify:
1
Item 1 has been marked as completed
```
### View a specific task
```
Please choose a command:
▸ Display an item
Enter the ID of the item to display:
1
Buy groceries [✅]
    Milk, eggs, bread
    Created: 2025-06-13 10:45:21
```
### Delete a task
```
Please choose a command:
▸ Delete an item
Enter the ID of the item to delete:
1
Deleted: Buy groceries [✅]
    Milk, eggs, bread
```