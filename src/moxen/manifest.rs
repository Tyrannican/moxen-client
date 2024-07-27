use crate::common::MoxenError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, fs, io::Write, path::Path};

// TODO: Add Categories
// TODO: Add support for Collections (e.g. DBM, etc.)
// TODO: Add support for Libraries (e.g. AceConsole, etc)

const MANIFEST: &'static str = "Moxen.toml";

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn load(dir: impl AsRef<Path>) -> Result<Self> {
        let contents =
            fs::read_to_string(dir.as_ref().join(MANIFEST)).context("reading manifest file");

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

    pub fn fresh(name: &str) -> Self {
        let manifest = Self {
            mox: Metadata {
                name: name.to_string(),
                version: Some("0.1.0".to_string()),
                description: "New World of Warcraft addon".to_string(),
                wow_version: "<Insert current WoW version here (11.0.1)!>".to_string(),
                authors: vec![],
                homepage: None,
                repository: None,
            },
            collection: None,
        };

        manifest
    }

    pub fn write(&self, dir: impl AsRef<Path>) -> Result<()> {
        let manifest = dir.as_ref().join(MANIFEST);
        let str_contents = toml::to_string(&self)?;
        std::fs::write(manifest, str_contents)?;

        Ok(())
    }

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

pub fn bootstrap_lua(dir: impl AsRef<Path>) -> Result<()> {
    let lua_path = dir.as_ref().join("start.lua");
    let mut f = std::fs::File::create(lua_path)?;
    f.write_all("print('Hello, World!')\n".as_bytes())?;
    Ok(())
}

pub fn bootstrap_toc(dir: impl AsRef<Path>, manifest: &PackageManifest) -> Result<()> {
    let mox = &manifest.mox;
    let mut normalised_name = mox.name.replace(" ", "");
    normalised_name.push_str(".toc");
    let toc_path = dir.as_ref().join(normalised_name);
    let mut f = std::fs::File::create(toc_path)?;

    f.write(
        format!("## Interface: <Current World of Warcraft Version Here (e.g. 110001)>\n")
            .as_bytes(),
    )?;
    let mox_version = if let Some(version) = &mox.version {
        version
    } else {
        &"0.1.0".to_string()
    };
    f.write(format!("## Version: {mox_version}\n").as_bytes())?;

    f.write(format!("## Title: {}\n", mox.name).as_bytes())?;
    f.write("## Notes: Created with Moxen\n".as_bytes())?;

    let author = mox.authors.join(",");
    f.write(format!("## Author: {author}\n").as_bytes())?;
    f.write("\n".as_bytes())?;
    f.write("start.lua\n".as_bytes())?;

    Ok(())
}
