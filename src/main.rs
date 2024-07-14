use clap::Parser;
use cli::{Cli, Commands};
use cmd::create::create_project;

mod cli;
mod cmd;
mod error;
mod utils;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create { name }) => match create_project(name) {
            Ok(_) => todo!(),
            Err(err) => {
                println!("{}", err);
            }
        },
        None => {}
    }
}
