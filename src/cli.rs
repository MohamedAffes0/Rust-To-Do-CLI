
use crate::to_do::{ToDo, Error};
use crate::to_do_list::ToDoList;
use std::io::Write;

pub enum CliCommand {
    /// Display an element.
    Get(usize),
    /// Display the entire list.
    Display,
    /// Change an element's state.
    Modify{
        id: usize,
        completed: bool
    },
    /// Insert a new element.
    Insert(ToDo),
    /// Delete an element.
    Delete(usize),
    Exit,
}

pub enum CliError {
    InvalidCommand,
    InvalidInput(Error),
}

pub fn get_command() -> Result<CliCommand, CliError> {
    println!("Please choose a command:");
    println!("1. Display an item");
    println!("2. Display the entire list");
    println!("3. Modify the state of an item");
    println!("4. Insert a new item");
    println!("5. Delete an item");
    println!("6. Exit");

    print!("Enter the command number: ");

    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    match input {
        "1" => {
            println!("Enter the ID of the item to display:");
            let mut id_input = String::new();
            std::io::stdin().read_line(&mut id_input).expect("Failed to read line");
            let id_input: usize = match id_input.trim().parse() {
                Ok(value) => value,
                Err(_) => return Err(CliError::InvalidCommand),
            };
            Ok(CliCommand::Get(id_input))
        },
        "2" => {
            Ok(CliCommand::Display)
        },
        "3" => {
            println!("Enter the ID of the item to modify:");
            let mut id_input = String::new();
            std::io::stdin().read_line(&mut id_input).expect("Failed to read line");
            let id_input: usize = id_input.trim().parse().expect("Invalid ID input");

            println!("Enter the new state (true for completed, false for not completed):");
            let mut state_input = String::new();
            std::io::stdin().read_line(&mut state_input).expect("Failed to read line");
            let completed: bool = state_input.trim().parse().expect("Invalid state input");

            Ok(CliCommand::Modify { id: id_input, completed })
        },
        "4" => {
            println!("Enter the title of the new item:");
            let mut title_input = String::new();
            std::io::stdin().read_line(&mut title_input).expect("Failed to read line");
            let title = title_input.trim().to_string();

            println!("Enter the description of the new item:");
            let mut description_input = String::new();
            std::io::stdin().read_line(&mut description_input).expect("Failed to read line");
            let description = description_input.trim().to_string();

            match ToDo::new(title, description) {
                Ok(todo) => Ok(CliCommand::Insert(todo)),
                Err(e) => Err(CliError::InvalidInput(e)), // Handle error appropriately
            }
        },
        "5" => {
            println!("Enter the ID of the item to delete:");
            let mut id_input = String::new();
            std::io::stdin().read_line(&mut id_input).expect("Failed to read line");
            let id_input: usize = id_input.trim().parse().expect("Invalid ID input");
            Ok(CliCommand::Delete(id_input))
        },
        "6" => {
            Ok(CliCommand::Exit)
        },
        _ => {
            Err(CliError::InvalidCommand)
        }
    }
}

impl CliCommand {
    pub fn execute(&self, todo_list: &mut ToDoList) -> Result<(), Error> {
        match self {
            CliCommand::Get(id) => {
                let todo = todo_list.get(*id - 1)?;
                println!("{}", todo);
                Ok(())
            },
            CliCommand::Display => {
                let size = todo_list.size();
                if size == 0 {
                    println!("The list is empty.");
                } else {
                    for i in 0..size {
                        match todo_list.get(i) {
                            Ok(todo) => {
                                println!("{}: {}", i + 1, todo);
                            },
                            Err(e) => {
                                return Err(e);
                            },
                        }
                    }
                }
                Ok(())
            },
            CliCommand::Modify { id, completed } => {
                todo_list.set_completed(*id - 1, *completed)?;
                println!("Item {} has been marked as {}", id, if *completed { "completed" } else { "not completed" });
                Ok(())
            },
            CliCommand::Insert(todo) => {
                println!("Inserted: {}", todo);
                todo_list.add(todo.clone());
                Ok(())
            },
            CliCommand::Delete(id) => {
                match todo_list.remove(*id - 1) {
                    Ok(todo) => {
                        println!("Deleted: {}", &todo);
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            },
            CliCommand::Exit => {
                println!("Exiting the application.");
                Ok(())
            },
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InvalidCommand => write!(f, "Invalid command entered. Please try again."),
            CliError::InvalidInput(e) => write!(f, "Invalid input: {}", e),
        }
    }
}