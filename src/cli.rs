use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run compiler from a file
    Run {
        /// Source of the file to execute
        #[clap(value_parser)]
        path: String,
    },
}
