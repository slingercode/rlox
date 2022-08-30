mod cli;
mod rlox;
mod token;
mod scanner;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli_instance = Cli::parse();
    let mut rlox_instance = rlox::Rlox { had_error: false };

    match cli_instance.command {
        Some(Commands::Run { path }) => {
            rlox_instance.run_file(path);
        }
        None => {
            rlox_instance.run_promt().unwrap();
        }
    }
}
