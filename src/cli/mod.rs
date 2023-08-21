use clap::{Parser, Subcommand};

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
        todo: String
    },
    /// Completes a todo
    Complete {
        #[arg()]
        id: String
    },
}

pub fn parse() {
    let args = Cli::parse();

    match args.command {
        Commands::Add { todo } => {
            println!("Adding todo {todo}");
        },
        Commands::Delete { id } => {
            println!("Removing todo w/ id {id}");
        },
        Commands::Update { id, todo } => {
            println!("Updating todo w/ id: {id} w/ data: {todo}");
        },
        Commands::Complete { id } => {
            println!("Completing todo w/ id {id}");
        },
    }
}
