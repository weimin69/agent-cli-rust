mod cli;
mod commands;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<(), String> {
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
    }
    Ok(())
}
