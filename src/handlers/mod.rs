/// Functions for performing operations on todos\

use crate::adapters;
use crate::domain;


/// Add a new todo to a todo list
pub fn add_todo<T: adapters::Repository>(todo: String, repo: &mut T) {
    let todo = domain::Todo::new(todo);

    repo.add(todo);
}

pub fn list<T: adapters::Repository>(repo: &mut T) -> Vec<domain::Todo> {
    println!("Listing all todos");
    repo.list().to_owned()
}

pub fn update_todo(id: String, todo: String) {
    println!("Updating todo w/ id: {id} w/ data: {:?}", todo);
}

pub fn start(id: String) {
    println!("Starting todo w/ id {id}");
}

pub fn complete(id: String) {
    println!("Completing todo w/ id {id}");
}

pub fn delete_todo(id: String) {
    println!("Removing todo w/ id {id}");
}

#[cfg(test)]
mod tests {
    use crate::adapters::Repository;

    use super::*;

    // Test svc layer
    #[test]
    fn test_adding_new_todo() {
        // Given
        let todo = String::from("Test my code");
        let todo_cp = todo.clone();
        let mut repo = adapters::InMemoryRepository::new();

        // When
        add_todo(todo, &mut repo);
        let first_todo = repo.list().iter();
        let todo = first_todo.as_ref().first().unwrap();

        // Then
        assert_eq!(todo.item(), todo_cp);
    }
}
