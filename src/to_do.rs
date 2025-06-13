#[derive(Clone)]
pub struct ToDo {
    title: String,
    description: String,
    completed: bool,
}

pub enum Error {
    EmptyField(EmptyFieldError),
    // MaxReached,
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
        write!(f, "ToDo: {{ title: {}, description: {}, completed: {} }}", self.title, self.description, self.completed)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyField(e) => match e {
                EmptyFieldError::Title => write!(f, "Title cannot be empty."),
                EmptyFieldError::Description => write!(f, "Description cannot be empty."),
            },
            // Error::MaxReached => write!(f, "Maximum number of ToDos reached."),
            Error::IndexOutOfBounds => write!(f, "Index out of bounds."),
        }
    }
}