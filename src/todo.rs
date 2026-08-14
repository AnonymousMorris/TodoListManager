use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub value: String,
    pub selected: bool,
    pub completed: bool, 
    pub editing: bool,
    pub description: String,
    pub todo_idx: usize,
}

impl Todo {
    pub fn new(idx: usize) -> Todo {
        Todo{
            selected: false,
            value: String::new(),
            completed: false,
            description: String::new(),
            editing: false,
            todo_idx: idx,
        }
    }
}
