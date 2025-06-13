use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize)]
pub struct ToDo {
    title: String,
    description: String,
    completed: bool,
    creation_date: DateTime<Local>,
}

pub enum Error {
    EmptyField(EmptyFieldError),
    IndexOutOfBounds
}

pub enum EmptyFieldError {
    Title,
    Description,
}

impl ToDo {
    pub fn new(title: String, description: String) -> Result<ToDo, Error> {
        if title.is_empty() {
            Err(Error::EmptyField(EmptyFieldError::Title))
        }
        else if description.is_empty()  {
            Err(Error::EmptyField(EmptyFieldError::Description))
        }else {
            let now = Local::now();
            Ok(ToDo {title, description, completed: false, creation_date: now })
        }
    }

    pub fn toggle_completion(&mut self) {
        self.completed = !self.completed;
    }

    pub fn get_completion(&self) -> bool {
        self.completed
    }

}

impl std::fmt::Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_icon = if self.completed { "✅" } else { "❌" };
        write!(
            f,
            "{} [{}]\n    {}\n    Created: {}\n",
            self.title,
            status_icon,
            self.description,
            self.creation_date.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyField(e) => match e {
                EmptyFieldError::Title => write!(f, "Title cannot be empty."),
                EmptyFieldError::Description => write!(f, "Description cannot be empty."),
            },
            Error::IndexOutOfBounds => write!(f, "Index out of bounds."),
        }
    }
}