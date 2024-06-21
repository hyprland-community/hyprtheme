// use crate::theme::{helper::{create_theme_id, ThemeId}};
use serde::Deserialize;
use std::path::PathBuf;

use anyhow::Result;


#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    pub desc: String,
    pub version: String,
    pub author: String,
    pub repo: String,
    pub branch: Option<String>,

    pub theme: ConfigTheme,
    pub hypr_module: Option<Vec<ConfigHyprModule>>,
    pub module: Option<Vec<ConfigModule>>,
    pub link: Option<Vec<ConfigLink>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigTheme {
    pub config: PathBuf,
    pub minimum_hyprland_version: String,

    pub load: Option<String>,
    pub unload: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigHyprModule {
    pub name: String,
    pub desc: String,

    pub enabled: Option<bool>,

    pub config: PathBuf,

    pub minimum_hyprland_version: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigModule {
    pub enabled:Option<bool>,

    pub config: PathBuf,

    pub minimum_hyprland_version: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigLink {
    pub from: PathBuf,
    pub to: PathBuf,
    pub ignore: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
}

impl Config {
    pub fn from_toml_str(toml: &str) -> Result<Self> {
        match toml::from_str(toml) {
            Ok(config) => Ok(config),
            Err(e) => Err(anyhow::anyhow!("Failed to parse theme config file: {}", e)),
        }
    }

    pub fn from_toml_file(file: &PathBuf) -> Result<Self> {
        match std::fs::read_to_string(file) {
            Ok(toml) => Self::from_toml_str(&toml),
            Err(e) => Err(anyhow::anyhow!("Failed to read theme config file: {}", e)),
        }
    }
}