mod cli;
mod commands;
use clap::Parser;
use crate::cli::{Cli,Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Hello => {
           commands::hello::execute();
        }
        Commands::Version => {
            commands::version::execute();
        }

    }
}
