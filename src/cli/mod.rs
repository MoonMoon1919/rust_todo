use clap::{Parser, Subcommand};

use crate::adapters;
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


pub fn parse<T: adapters::Repository>(repo: &mut T) {
    let args = Cli::parse();

    match args.command {
        Commands::Add { todo } => {
            handlers::add_todo(todo, repo);
        },
        Commands::Delete { id } => {
            handlers::delete_todo(id);
        },
        Commands::Update { id, todo } => {
            handlers::update_todo(id, todo);
        },
        Commands::List => {
            handlers::list();
        },
        Commands::Start { id } => {
            handlers::start(id);
        }
        Commands::Complete { id } => {
            handlers::complete(id);
        },
    }
}
