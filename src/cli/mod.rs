use clap::{Parser, Subcommand};

use crate::handlers;

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "A todo app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds a new todo
    #[command(arg_required_else_help = true)]
    Add {
        /// The todo to add
        todo: String
    },
    /// Deletes a todo
    Delete {
        #[arg()]
        id: String
    },
    /// Updates a todo
    Update {
        #[arg()]
        id: String,

        #[arg()]
        todo: String,
    },
    /// Lists all todos
    List,
    /// Starts a todo
    Start {
        #[arg()]
        id: String
    },
    /// Completes a todo
    Complete {
        #[arg()]
        id: String
    },
}

// Temp data store
pub struct InMemoryRepository {
    todos: Vec<handlers::Todo>
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository { todos: vec![] }
    }
}

impl handlers::Repository for InMemoryRepository {
    fn add(&self, todo: handlers::Todo) {}

    fn get(&self, id: String) {}
}

pub fn parse() {
    let args = Cli::parse();
    let repo = InMemoryRepository::new();

    match args.command {
        Commands::Add { todo } => {
            handlers::add_todo(todo, &repo);
        },
        Commands::Delete { id } => {
            handlers::delete_todo(id, &repo);
        },
        Commands::Update { id, todo } => {
            handlers::update_todo(id, todo, &repo);
        },
        Commands::List => {
            handlers::list(&repo);
        },
        Commands::Start { id } => {
            handlers::start(id, &repo);
        }
        Commands::Complete { id } => {
            handlers::complete(id, &repo);
        },
    }
}
