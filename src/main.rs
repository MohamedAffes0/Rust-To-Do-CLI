mod to_do;
mod to_do_list;
mod cli;

use to_do_list::ToDoList;
use cli::{get_command, CliCommand};

fn main() {
    let mut todo_list = ToDoList::new();
    println!("Welcome to the To-Do List Application!");
    loop {
        match get_command() {
            Ok(command) => {
                match command {
                    CliCommand::Exit => {
                        match command.execute(&mut todo_list) {
                            Err(e) => eprintln!("Error: {}", e),
                            _ => {},
                        }
                        break;
                    },
                    _ => {
                        match command.execute(&mut todo_list) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    },
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            },
        }
    }
}
