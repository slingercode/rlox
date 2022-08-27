use clap::Parser;
use cli::{Cli, Commands};

mod cli;

fn main() {
    let cli_instance = Cli::parse();

    match cli_instance.command {
        Some(Commands::Run { src }) => {
            println!("runFile: {}", src);
        }
        None => {
            println!("runPrompt");
        }
    }
}
