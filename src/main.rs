pub mod cli;
mod handlers;
mod adapters;
mod domain;

fn main() {

    let mut repo = adapters::FileSystemRepository::new();

    cli::parse(&mut repo);
}
