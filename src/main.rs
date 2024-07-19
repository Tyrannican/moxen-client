use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod cli;
mod common;
mod manifest;
mod package;
// mod deploy;

use cli::*;
use manifest::load_manifest;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let dir = if let Some(dir) = cli.directory {
        PathBuf::from(dir)
    } else {
        std::env::current_dir()?
    };

    println!("Current dir: {dir:?}");
    match cli.commands {
        Commands::Info => {
            let manifest = load_manifest(dir)?;
            println!("{manifest}");
        }
        Commands::Package => {
            let manifest = load_manifest(&dir)?;
            package::package(manifest, dir).await?;
        }
    }

    Ok(())
}
