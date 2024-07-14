use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// create a new project
    #[command(alias="new")]
    Create {
        name: Option<String>,
    },
}