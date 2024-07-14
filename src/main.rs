use clap::Parser;
use cli::{Cli, Commands};
use cmd::create::create_project;

mod cli;
mod utils;
mod cmd;
mod error;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create { project_name }) => {
            
            create_project(project_name);
        }
        None => {}
    }
}
