use anyhow::Result;
use clap::Parser;

mod cli;
mod common;
mod moxen;

use cli::*;
use moxen::Manager;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut pkg_mgr = Manager::new(cli.directory);

    match cli.commands {
        Commands::New { name } => {
            pkg_mgr.bootstrap(name)?;
        }
        Commands::Info => {
            pkg_mgr.load()?;
            pkg_mgr.info()?;
        }
        Commands::Package => {
            pkg_mgr.load()?;
            pkg_mgr.package()?;
        }
        Commands::Publish => {
            pkg_mgr.publish().await?;
        }
        Commands::Convert => {
            pkg_mgr.add_manifest()?;
        }
        Commands::Clean => {
            pkg_mgr.clean()?;
        }
    }

    Ok(())
}
