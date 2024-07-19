use crate::common::ScholoError;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fmt, fs, path::Path};

// TODO: Add Categories

const MANIFEST: &'static str = "Scholo.toml";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Addon {
    pub addon: AddonMetadata,

    // TODO: Deal with at some point
    pub package: Option<PackageInformation>,
}

#[derive(Debug, Deserialize)]
pub struct AddonMetadata {
    pub name: String,
    pub version: Option<String>,
    pub wow_version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PackageInformation {
    #[serde(rename = "ignore")]
    ignore_list: Option<Vec<String>>,
}

impl fmt::Display for Addon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addon = &self.addon;

        writeln!(f, "Name: {}", addon.name)?;
        if let Some(version) = &addon.version {
            writeln!(f, "Addon Version: {version}")?;
        }
        writeln!(f, "World of Warcraft Version: {}", addon.wow_version)?;
        writeln!(f, "Addon Description: \"{}\"", addon.description)?;
        writeln!(f, "Authors:")?;
        for author in &addon.authors {
            writeln!(f, "- {author}")?;
        }

        if let Some(homepage) = &addon.homepage {
            writeln!(f, "Home: {homepage}")?;
        }

        if let Some(repo) = &addon.repository {
            writeln!(f, "Source Code: {repo}")?;
        }

        Ok(())
    }
}

pub fn load_manifest(dir: impl AsRef<Path>) -> Result<Addon> {
    let contents = fs::read_to_string(dir.as_ref().join(MANIFEST)).context("reading manifest file");

    match contents {
        Ok(contents) => {
            let manifest: Addon =
                toml::from_str(&contents).context("deserializing manifest file")?;

            Ok(manifest)
        }
        Err(err) => {
            eprintln!("No Scholo.toml file found in project root: {err}");
            anyhow::bail!(ScholoError::MissingManifestFile);
        }
    }
}
