mod cli;
mod rlox;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli_instance = Cli::parse();

    match cli_instance.command {
        Some(Commands::Run { path }) => {
            rlox::run_file(path);
        }
        None => {
            rlox::run_promt().unwrap();
        }
    }
}
