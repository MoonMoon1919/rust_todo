use crate::domain;

pub trait Repository {
    fn add(&mut self, todo: domain::Todo) {}
    fn get(&self, id: String) -> &domain::Todo {
        panic!("not implemented")
    }
    fn list(&self) -> &Vec<domain::Todo> {
        panic!("not implemented")
    }
}

pub struct InMemoryRepository {
    todos: Vec<domain::Todo>
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository { todos: vec![] }
    }
}

impl Repository for InMemoryRepository {
    fn add(&mut self, todo: domain::Todo) {
        self.todos.push(todo);
    }

    fn get(&self, id: String) -> &domain::Todo {
        let filtered = &self.todos.iter().filter(|t| t.id() == id).next();

        let result = match filtered {
            Some(res) => res,
            _ => panic!("Error todo not found")
        };

        result
    }

    fn list(&self) -> &Vec<domain::Todo> {
        &self.todos
    }
}
