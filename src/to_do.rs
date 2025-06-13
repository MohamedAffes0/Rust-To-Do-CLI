use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ToDo {
    title: String,
    description: String,
    completed: bool,
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
            Ok(ToDo {title, description, completed: false})
        }
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}

impl std::fmt::Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_icon = if self.completed { "✅" } else { "❌" };
        write!(
            f,
            "{} [{}]\n    {}\n",
            self.title,
            status_icon,
            self.description
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