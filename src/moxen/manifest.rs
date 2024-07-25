use crate::common::MoxenError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, fs, path::Path};

// TODO: Add Categories
// TODO: Add support for Collections (e.g. DBM, etc.)
// TODO: Add support for Libraries (e.g. AceConsole, etc)

const MANIFEST: &'static str = "Moxen.toml";

#[derive(Debug, Deserialize)]
pub struct PackageManifest {
    pub mox: Metadata,
    pub collection: Option<PackageCollection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: Option<String>,
    pub wow_version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCollection {
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalizedManifest {
    name: String,
    version: Option<String>,
    wow_version: String,
    cksum: String,
}

#[allow(dead_code)]
impl PackageManifest {
    pub fn normalise_name(&self) -> String {
        let mut name = self.mox.name.to_lowercase().replace(" ", "-");
        name.push('-');
        if let Some(version) = &self.mox.version {
            name.push_str(&version);
        } else {
            name.push_str(&self.mox.wow_version);
        }

        name
    }

    pub fn normalise(self, cksum: String) -> NormalizedManifest {
        let name = self.normalise_name();
        NormalizedManifest {
            name,
            version: self.mox.version,
            wow_version: self.mox.wow_version,
            cksum,
        }
    }
}

impl fmt::Display for PackageManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addon = &self.mox;

        writeln!(f, "--- Mox Manifest ---")?;
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

        if let Some(collection) = &self.collection {
            writeln!(f, "\nCollection:")?;
            for item in collection.members.iter() {
                writeln!(f, "- {item}")?;
            }
        }

        writeln!(f, "------")?;

        Ok(())
    }
}

pub fn load_manifest(dir: impl AsRef<Path>) -> Result<PackageManifest> {
    let contents = fs::read_to_string(dir.as_ref().join(MANIFEST)).context("reading manifest file");

    match contents {
        Ok(contents) => {
            let manifest: PackageManifest =
                toml::from_str(&contents).context("deserializing manifest file")?;

            Ok(manifest)
        }
        Err(err) => {
            eprintln!("No Moxen.toml file found in project root: {err}");
            anyhow::bail!(MoxenError::MissingManifestFile);
        }
    }
}
