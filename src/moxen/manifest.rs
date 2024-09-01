use crate::common::MoxenError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, fs, io::Write, path::Path};

static MANIFEST: &str = "Moxen.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub mox: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: Option<String>,
    pub wow_version: String,
    pub description: String,
    pub categories: Option<Vec<MoxCategory>>,
    pub authors: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalizedManifest {
    pub name: String,
    pub version: Option<String>,
    pub wow_version: String,
    pub categories: Vec<MoxCategory>,
    pub cksum: String,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MoxCategory {
    Achievements,
    ActionBars,
    Artwork,
    AuctionEconomy,
    AudioVideo,
    BagsInventory,
    BossEncounters,
    BuffsDebuffs,
    ChatCommunication,
    Class,
    Combat,
    Companions,
    DataExport,
    DevelopmentTools,
    Garrison,
    Guild,
    Library,
    Mail,
    MapMinimap,
    Minigames,
    #[default]
    Miscellaneous,
    Plugins,
    Professions,
    Pvp,
    QuestsLevelling,
    Roleplay,
    Tooltip,
    TwitchIntegration,
    UnitFrames,
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
                description: "Bootstrapped by Moxen".to_string(),
                wow_version: "<Insert current WoW version here (11.0.1)!>".to_string(),
                categories: Some(vec![MoxCategory::Miscellaneous]),
                authors: vec![],
                homepage: None,
                repository: None,
                dependencies: None,
                ignore: None,
            },
        };

        manifest
    }

    pub fn interactive(name: &str) -> PackageManifest {
        println!("Creating Moxen.toml ({name})...");
        let mut manifest = Self::fresh(name);
        let version = get_user_input("Addon Version");
        manifest.mox.version = Some(version);

        let description = get_user_input("Addon Description");
        manifest.mox.description = description;

        let wow_version = get_user_input("WoW version (e.g. 11.0.1)");
        manifest.mox.wow_version = wow_version;

        let authors = get_user_input("Authors (separated by ,)");
        let authors = authors.split(',').map(|a| a.trim().to_owned()).collect();
        manifest.mox.authors = authors;

        let homepage = get_user_input("Homepage (optional)");
        if !homepage.is_empty() {
            manifest.mox.homepage = Some(homepage);
        }

        let repository = get_user_input("Repository (git, svn, etc. - optional)");
        if !repository.is_empty() {
            manifest.mox.repository = Some(repository);
        }

        manifest
    }

    pub fn write(&self, dir: impl AsRef<Path>) -> Result<()> {
        let manifest = dir.as_ref().join(MANIFEST);
        let str_contents = toml::to_string(&self)?;
        std::fs::write(manifest, str_contents)?;

        Ok(())
    }

    pub fn normalise_name(&self, with_version: bool) -> String {
        let mut name = self.mox.name.to_lowercase().replace(" ", "-");
        if !with_version {
            return name;
        }

        name.push('-');
        if let Some(version) = &self.mox.version {
            name.push_str(&version.to_lowercase());
        } else {
            name.push_str(&self.mox.wow_version);
        }

        name
    }

    pub fn normalise(self, cksum: String) -> NormalizedManifest {
        let name = self.normalise_name(false);
        let categories = match self.mox.categories {
            Some(cat) => {
                if !cat.is_empty() {
                    cat
                } else {
                    vec![MoxCategory::Miscellaneous]
                }
            }
            None => vec![MoxCategory::Miscellaneous],
        };

        NormalizedManifest {
            name,
            version: self.mox.version,
            wow_version: self.mox.wow_version,
            categories,
            cksum,
        }
    }

    pub fn add_dependency(&mut self, dep: String) {
        if let Some(deps) = self.mox.dependencies.as_mut() {
            if deps.contains(&dep) {
                return;
            }
            deps.push(dep);
        } else {
            let deps = vec![dep];
            self.mox.dependencies = Some(deps);
        }
    }
}

fn get_user_input(msg: &str) -> String {
    let mut buf = String::new();
    print!("{msg}: ");
    let stdin = std::io::stdin();
    let _ = std::io::stdout().flush();
    stdin.read_line(&mut buf).expect("unable to parse stdin");

    buf.trim().to_owned()
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

        if !&addon.authors.is_empty() {
            writeln!(f, "Authors:")?;
            for author in &addon.authors {
                writeln!(f, "- {author}")?;
            }
        }

        if let Some(categories) = &addon.categories {
            if !categories.is_empty() {
                writeln!(f, "Categories:")?;
            }
            for category in categories.iter() {
                writeln!(f, "- {category}")?;
            }
        }

        if let Some(homepage) = &addon.homepage {
            writeln!(f, "Home: {homepage}")?;
        }

        if let Some(repo) = &addon.repository {
            writeln!(f, "Source Code: {repo}")?;
        }

        writeln!(f, "------")?;

        Ok(())
    }
}

impl std::fmt::Display for MoxCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Achievements => write!(f, "Achievements"),
            Self::ActionBars => write!(f, "Action Bars"),
            Self::Artwork => write!(f, "Artwork"),
            Self::AuctionEconomy => write!(f, "Auction & Economy"),
            Self::AudioVideo => write!(f, "Audio & Visual"),
            Self::BagsInventory => write!(f, "Bags & Inventory"),
            Self::BossEncounters => write!(f, "Boss Encounters"),
            Self::BuffsDebuffs => write!(f, "Buffs & Debuffs"),
            Self::ChatCommunication => write!(f, "Chat & Communication"),
            Self::Class => write!(f, "Class"),
            Self::Combat => write!(f, "Combat"),
            Self::Companions => write!(f, "Companions"),
            Self::DataExport => write!(f, "Data Export"),
            Self::DevelopmentTools => write!(f, "Development Tools"),
            Self::Garrison => write!(f, "Garrison"),
            Self::Guild => write!(f, "Guild"),
            Self::Library => write!(f, "Library"),
            Self::Mail => write!(f, "Mail"),
            Self::MapMinimap => write!(f, "Map & Minimap"),
            Self::Minigames => write!(f, "Minigames"),
            Self::Miscellaneous => write!(f, "Miscellaneous"),
            Self::Plugins => write!(f, "Plugins"),
            Self::Professions => write!(f, "Professions"),
            Self::Pvp => write!(f, "PvP"),
            Self::QuestsLevelling => write!(f, "Quests & Levelling"),
            Self::Roleplay => write!(f, "Roleplay"),
            Self::Tooltip => write!(f, "Tooltip"),
            Self::TwitchIntegration => write!(f, "Twitch Integration"),
            Self::UnitFrames => write!(f, "Unit Frames"),
        }
    }
}

pub fn bootstrap_lua(dir: impl AsRef<Path>) -> Result<()> {
    let lua_path = dir.as_ref().join("start.lua");
    let mut f = std::fs::File::create(lua_path)?;
    f.write_all("print('Hello, World!')\n".as_bytes())?;
    Ok(())
}

pub fn bootstrap_gitignore(dir: impl AsRef<Path>) -> Result<()> {
    let ignore = dir.as_ref().join(".gitignore");
    let mut f = std::fs::File::create(ignore)?;
    f.write_all("docs/".as_bytes())?;

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
