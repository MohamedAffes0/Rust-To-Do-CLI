use crate::to_do::{ToDo, Error};

pub struct ToDoList(Vec<ToDo>);

impl ToDoList {
    pub fn new() -> ToDoList {
        ToDoList(Vec::new())
    }

    pub fn add(&mut self, todo: ToDo) {
        self.0.push(todo);
    }

    pub fn remove(&mut self, id: usize) -> Result<ToDo,Error>{
        if id >= self.0.len(){
            return Err(Error::IndexOutOfBounds);
        }
        Ok(self.0.remove(id as usize))
    }

    pub fn get(&self, id: usize) -> Result<&ToDo, Error> {
        match self.0.get(id) {
            Some(todo) => Ok(todo),
            None => Err(Error::IndexOutOfBounds)
        }
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn set_completed(&mut self, id: usize, completed: bool) -> Result<(), Error> {
        match self.0.get_mut(id) {
            Some(todo) => {
                todo.set_completed(completed);
                Ok(())
            },
            None => Err(Error::IndexOutOfBounds)
        }
    }
}
