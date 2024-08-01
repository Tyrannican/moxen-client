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

    /// Add a library as a dependency of this addon
    Add {
        /// Name of the library to add
        #[clap(value_delimiter = ' ')]
        names: Vec<String>,
    },

    /// Display information about a Moxen project
    Info,

    /// Package a Moxen project ready for publishing
    Package,

    /// Publish a Moxen project to the registry
    Publish,

    /// Add a Moxen.toml manifest to an existing project
    Moxify,

    /// Clean any packaged artifacts
    Clean,
}
