mod cli;

use clap::Parser;
use crate::cli::Cli;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        crate::cli::Commands::Hello => {
            println!("Hello,Agent CLI!");

        }

    }
}
