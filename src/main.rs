mod cli;
mod commands;
use clap::Parser;
use crate::cli::{Cli,Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Hello {name,age} => {
           commands::hello::execute(name,age);
        }
        Commands::Version => {
            commands::version::execute();
        }

    }
}
