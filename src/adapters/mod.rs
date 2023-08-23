use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::io::BufReader;
use std::path;

use serde_json;

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

pub struct FileSystemRepository {
    todos: HashMap<String, domain::Todo>,
    db_file: String
}

impl FileSystemRepository {
    pub fn new() -> Self {
        let file = File::open(path::Path::new("db.json")).unwrap();
        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader);

        let loaded_map = match data {
            Ok(d) => {
                println!("{:?}", d);
                d
            },
            _ => HashMap::new(),
        };

        FileSystemRepository {
            todos: loaded_map,
            db_file: String::from("db.json")
        }
    }

    fn check_or_create_db(&self, path: &path::Path) {
        let exists = path.exists();

        if !exists {
            let _ = File::create(path);
        }
    }

    fn flush(&self) {
        self.check_or_create_db(&path::Path::new(&self.db_file));

        let writer = OpenOptions::new()
            .write(true)
            .append(false)
            .open(&self.db_file)
            .unwrap();

        let _ = serde_json::to_writer(writer, &self.todos);
    }
}

impl Repository for FileSystemRepository {
    fn add(&mut self, todo: domain::Todo) {
        self.todos.insert(todo.id().to_owned(), todo);

        self.flush();
    }

    fn get(&self, id: &String) -> domain::Todo {
        self.todos.get(id).unwrap().to_owned()
    }

    fn list(&self) -> Vec<domain::Todo> {
        Vec::from_iter(self.todos.values().cloned())
    }

    fn delete(&mut self, id: &String) {
        self.todos.remove(id);

        self.flush();
    }
}
