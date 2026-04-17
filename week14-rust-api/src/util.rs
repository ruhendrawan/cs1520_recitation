use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type TodoList = HashMap<String, Todo>;
pub type SharedTodoList = Arc<RwLock<TodoList>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct NewTodo {
    // Recitation task 1:
    // add `due_date: Option<String>` here.
    // Suggested string format: "YYYY-MM-DD"
    task: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    // Recitation task 1:
    // add the same optional due-date field here too so GET/PUT responses include it.
    task: String,
    #[serde(default)]
    pub done: bool,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
pub struct TodoWithId {
    // This response type is useful for the sorted endpoint because a `Todo`
    // does not include the HashMap key.
    pub id: String,
    #[serde(flatten)]
    pub todo: Todo,
}

impl Todo {
    pub fn new(task: String) -> Self {
        // If you add a due-date field, update this constructor too.
        Self { task, done: false }
    }
}

impl From<NewTodo> for Todo {
    fn from(item: NewTodo) -> Self {
        // If you add a due-date field to `NewTodo`, copy it into `Todo` here.
        Self {
            task: item.task,
            done: false,
        }
    }
}
