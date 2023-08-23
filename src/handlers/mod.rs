/// Functions for performing operations on todos\

use crate::adapters;
use crate::domain;


/// Add a new todo to a todo list
pub fn add<T: adapters::Repository>(todo: String, repo: &mut T) -> String {
    let todo = domain::Todo::new(todo);
    let id = todo.clone().id().to_owned();

    repo.add(todo);

    id
}

pub fn list<T: adapters::Repository>(repo: &mut T) -> Vec<domain::Todo> {
    repo.list().to_owned()
}

pub fn update<T: adapters::Repository>(id: &String, todo: String, repo: &mut T) {
    let mut item: domain::Todo = repo.get(id);

    item.update_item(todo);

    repo.add(item)
}

pub fn start<T: adapters::Repository>(id: &String, repo: &mut T) {
    let mut item: domain::Todo = repo.get(id);

    item.start();

    repo.add(item)
}

pub fn complete<T: adapters::Repository>(id: &String, repo: &mut T) {
    let mut item: domain::Todo = repo.get(id);

    item.complete();

    repo.add(item)
}

pub fn delete<T: adapters::Repository>(id: &String, repo: &mut T) {
    repo.delete(id);
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
        add(todo, &mut repo);
        let first_todo = repo.list();
        let todo = first_todo.first().unwrap();

        // Then
        assert_eq!(todo.item(), todo_cp);
    }

    #[test]
    fn test_listing_todos() {
        // Given
        let todo = String::from("Test my code");
        let mut repo = adapters::InMemoryRepository::new();
        add(todo, &mut repo);

        // When
        let todos = list(&mut repo);

        // Then
        assert_eq!(todos.len(), 1)
    }

    #[test]
    fn test_updating_todo() {
        // Given
        let todo = String::from("Test my code");
        let mut repo = adapters::InMemoryRepository::new();
        let id = add(todo, &mut repo);
        let updated_todo = String::from("Really test my code");
        let cloned_update = updated_todo.clone();

        // When
        update(&id, updated_todo, &mut repo);

        // Then
        let todo = repo.get(&id);
        assert_eq!(todo.item(), cloned_update);
    }

    #[test]
    fn test_starting_todo() {
        // Given
        let todo = String::from("Test my code");
        let mut repo = adapters::InMemoryRepository::new();
        let id = add(todo, &mut repo);

        // When
        start(&id, &mut repo);

        // Then
        let todo = repo.get(&id);
        assert_eq!(todo.status().to_owned(), domain::Status::InProgress);
    }

    #[test]
    fn test_completing_todo() {
        // Given
        let todo = String::from("Test my code");
        let mut repo = adapters::InMemoryRepository::new();
        let id = add(todo, &mut repo);

        // When
        complete(&id, &mut repo);

        // Then
        let todo = repo.get(&id);
        assert_eq!(todo.status().to_owned(), domain::Status::Completed);
    }
}
