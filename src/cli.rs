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

        #[clap(long, default_value = None)]
        docs: Option<DocumentationType>,
    },

    /// Add a package as a dependency of this addon/library
    Add {
        /// Name of the package to add
        #[clap(value_delimiter = ' ')]
        names: Vec<String>,
    },

    /// Display information about a Moxen project
    Info,

    /// Package a Moxen project ready for publishing
    Package,

    /// Publish a Moxen project to the registry
    Publish,

    /// Register to the Moxen registry
    Register {
        /// Username for the registry
        name: String,
    },

    /// Recover an account with a recovery code
    Recover {
        /// Username to recover
        name: String,

        /// Valid recovery code for the account
        recovery_code: String,
    },

    /// Add a Moxen.toml manifest to an existing project
    Moxify,

    /// Clean any packaged artifacts
    Clean,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum DocumentationType {
    Live,
    Classic,
    Vanilla,
}

impl DocumentationType {
    pub fn repo(&self) -> String {
        match self {
            Self::Live => "https://github.com/Gethe/wow-ui-source/tree/live".to_string(),
            Self::Classic => "https://github.com/Gethe/wow-ui-source/tree/classic".to_string(),
            Self::Vanilla => "https://github.com/Gethe/wow-ui-source/tree/classic_era".to_string(),
        }
    }
}
