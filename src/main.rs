mod to_do;
mod to_do_list;
mod cli;

use to_do_list::ToDoList;
use cli::{get_command, CliCommand};

fn main() {
    let path = "todo_data.json";
    // Load the To-Do List from the file, or create a new one if it doesn't exist
    let mut todo_list = match ToDoList::load_from_file(path) {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error loading To-Do List: {}", e);
            ToDoList::new()
        }
    };

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
                        // save the To-Do List to the file before exiting
                        if let Err(e) = todo_list.save_to_file(path) {
                            println!("Error saving To-Do List: {}", e);
                        }
                        break;
                    },
                    _ => {
                        match command.execute(&mut todo_list) {
                            Ok(_) => {
                                // Save the To-Do List to the file after each command
                                if let Err(e) = todo_list.save_to_file(path) {
                                    println!("Error saving To-Do List: {}", e);
                                }
                            },
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
