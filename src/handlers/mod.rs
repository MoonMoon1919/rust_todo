/// Functions for performing operations on todos\
use uuid::Uuid;
use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

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
}

// Service layer
pub fn add_todo(todo: String) -> Todo {
    let todo = Todo::new(todo);

    println!("New todo w/ id {}, data {}, status {:?}", todo.id, todo.item, todo.status);

    todo
}

pub fn delete_todo(id: String) {
    println!("Removing todo w/ id {id}");
}

pub fn update_todo(id: String, todo: String) {
    println!("Updating todo w/ id: {id} w/ data: {:?}", todo);
}

pub fn list() {
    println!("Listing all the things you have to do");
}

pub fn start(id: String) {
    println!("Starting todo w/ id {id}");
}

pub fn complete(id: String) {
    println!("Completing todo w/ id {id}");
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

    // Test svc layer
    #[test]
    fn test_adding_new_todo() {
        // Given
        let todo = String::from("Test my code");
        let todo_cp = todo.clone();

        // When
        let new_todo = add_todo(todo);

        // Then
        assert_eq!(new_todo.item, todo_cp);
    }
}
