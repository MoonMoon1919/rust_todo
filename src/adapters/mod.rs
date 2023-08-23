use std::collections::HashMap;

use crate::domain;

pub trait Repository {
    fn add(&mut self, #[allow(unused)] todo: domain::Todo) {}
    fn get(&self, #[allow(unused)] id: &String) -> domain::Todo {
        panic!("not implemented")
    }
    fn list(&self) -> Vec<domain::Todo> {
        panic!("not implemented")
    }
    fn delete(&mut self, #[allow(unused)] id: &String) {}
}

pub struct InMemoryRepository {
    todos: HashMap<String, domain::Todo>
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository { todos: HashMap::new() }
    }
}

impl Repository for InMemoryRepository {
    fn add(&mut self, todo: domain::Todo) {
        self.todos.insert(todo.id().to_owned(), todo);
    }

    fn get(&self, id: &String) -> domain::Todo {
        self.todos.get(id).unwrap().to_owned()
    }

    fn list(&self) -> Vec<domain::Todo> {
        Vec::from_iter(self.todos.values().cloned())
    }

    fn delete(&mut self, id: &String) {
        self.todos.remove(id);
    }
}
