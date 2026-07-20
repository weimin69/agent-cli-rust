mod cli;
mod commands;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Hello { name, age } => {
            commands::hello::execute(name, age)?;
        }
        Commands::Version => {
            commands::version::execute()?;
        }
        Commands::Echo { words, upper } => {
            commands::echo::execute(&words, upper)?;
        }
        Commands::Divide { dividend, divisor } => {
            commands::divide::execute(dividend, divisor)?;
        }
        Commands::Sum { numbers } => {
            commands::sum::execute(&numbers)?;
        }
        Commands::Repeat { words, times } => {
            commands::repeat::execute(&words, times)?;
        }
    }
    Ok(())
}
