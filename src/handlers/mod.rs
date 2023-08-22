/// Functions for performing operations on todos\
use uuid::Uuid;
use clap::ValueEnum;

// Domain
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

// Adapters...
pub trait Repository {
    fn add(&self, todo: Todo) {}
    fn get(&self, id: String) {}
}



// Service layer
pub fn add_todo<T: Repository>(todo: String, repo: &T) -> Todo {
    let todo = Todo::new(todo);

    println!("New todo w/ id {}, data {}, status {:?}", todo.id, todo.item, todo.status);

    todo
}

pub fn delete_todo<T: Repository>(id: String, repo: &T) {
    println!("Removing todo w/ id {id}");
}

pub fn update_todo<T: Repository>(id: String, todo: String, repo: &T) {
    println!("Updating todo w/ id: {id} w/ data: {:?}", todo);
}

pub fn list<T: Repository>(repo: &T) {
    println!("Listing all the things you have to do");
}

pub fn start<T: Repository>(id: String, repo: &T) {
    println!("Starting todo w/ id {id}");
}

pub fn complete<T: Repository>(id: String, repo: &T) {
    println!("Completing todo w/ id {id}");
}

#[cfg(test)]
mod tests {
    pub struct InMemoryRepository {
        todos: Vec<Todo>
    }

    impl InMemoryRepository {
        pub fn new() -> Self {
            InMemoryRepository { todos: vec![] }
        }
    }

    impl Repository for InMemoryRepository {
        fn add(&self, todo: Todo) {}
        fn get(&self, id: String) {}
    }

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
        let repo = InMemoryRepository::new();

        // When
        let new_todo = add_todo(todo, &repo);

        // Then
        assert_eq!(new_todo.item, todo_cp);
    }
}
