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
    let mut pkg_mgr = Manager::new(cli.directory)?;

    match cli.commands {
        Commands::New { name, docs } => pkg_mgr.bootstrap(name, docs)?,
        Commands::Add { names } => pkg_mgr.download_dependencies(names).await?,
        Commands::Info => pkg_mgr.info(),
        Commands::Package => {
            pkg_mgr.package()?;
        }
        Commands::Register { name } => pkg_mgr.register(name).await?,
        Commands::Recover {
            name,
            recovery_code,
        } => pkg_mgr.recover(name, recovery_code).await?,
        Commands::Publish => pkg_mgr.publish().await?,
        Commands::Moxify => pkg_mgr.convert_to_mox()?,
        Commands::Clean => pkg_mgr.clean()?,
    }

    Ok(())
}
