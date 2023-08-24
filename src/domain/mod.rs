use std::fmt;
use uuid::Uuid;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    id: String,
    item: String,
    status: Status,
}

impl Todo {
    pub fn new(todo: String) -> Self {
        Todo {
            id: Uuid::new_v4().to_string(),
            item: todo,
            status: Status::NotStarted,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn item(&self) -> &str {
        &self.item
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn update_item(&mut self, item: String) -> &str {
        self.item = item;
        &self.item
    }

    pub fn start(&mut self) {
        self.status = Status::InProgress;
    }

    pub fn complete(&mut self) {
        self.status = Status::Completed;
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {} | {:?}", self.id, self.item, self.status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_new_todo() {
        // Given
        let todo = String::from("Test my code");
        let todo_cp = todo.clone();

        // When
        let new_todo = Todo::new(todo);

        // Then
        assert_eq!(new_todo.item, todo_cp);
    }
}
