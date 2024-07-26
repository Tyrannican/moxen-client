use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long)]
    pub directory: Option<String>,

    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        /// Name of the new project
        name: String,
    },
    Info,
    Package,
    Publish,
    Clean,
}
