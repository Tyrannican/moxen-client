use anyhow::Result;
use clap::Parser;

mod cli;
mod common;
mod moxen;
// mod deploy;

use cli::*;
use moxen::Manager;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut pkg_mgr = Manager::new(cli.directory);

    match cli.commands {
        Commands::Info => {
            pkg_mgr.load()?;
            pkg_mgr.info()?;
        }
        Commands::Package => {
            pkg_mgr.load()?;
            pkg_mgr.package().await?;
        }
        Commands::Clean => {
            pkg_mgr.clean()?;
        }
    }

    Ok(())
}
