use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "Moxen")]
#[clap(version = "0.1")]
#[clap(about = "World of Warcraft Addon development toolkit", long_about = None)]
pub struct Cli {
    /// DEBUG ONLY: Sets the directory to a test location
    #[clap(short, long)]
    pub directory: Option<String>,

    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new Moxen project
    New {
        /// Name of the new project
        name: String,
    },

    /// Display information about a Moxen project
    Info,

    /// Package a Moxen project ready for publishing
    Package,

    /// Publish a Moxen project to the registry
    Publish,

    /// Clean any packaged artifacts
    Clean,
}
