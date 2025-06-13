use crate::to_do::{ToDo, Error};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
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

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<ToDoList> {
        if !path.as_ref().exists() {
            // If the file does not exist, return an empty ToDoList
            return Ok(ToDoList(Vec::new()));
        }
        // Read the file and deserialize it into a ToDoList
        let data = fs::read_to_string(path)?;
        let list: ToDoList = serde_json::from_str(&data)?;
        Ok(list)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

}
