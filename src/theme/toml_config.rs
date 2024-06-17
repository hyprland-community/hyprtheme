use super::helper::{create_theme_id, ThemeId};
use serde::Deserialize;
use std::path::PathBuf;

/// Parsed theme config, does not have computed properties yet
///
/// For that see the Theme struct
#[derive(Deserialize, Debug)]
pub struct ParsedThemeConfig {
    pub meta: ThemeMeta,
    pub hypr: HyprConfig,
    // Let's make this optional, for people who only use scripts to setup their dots
    // Not optimal, but we don't have to feature those
    // TODO warn if dots is empty
    #[serde(default)]
    pub dots: Vec<DotsDirectoryConfig>,
    pub lifetime: Option<LifeTimeConfig>,
    // TODO install extra configs
    #[serde(default)]
    pub extra_configs: Vec<ExtraConfig>,
    // TODO install dependencies
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// The version of the hyprtheme.toml format
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct ThemeMeta {
    pub name: String,
    pub description: String,
    /// The version of the theme
    pub version: String,
    pub author: String,
    /// Git repository of the theme
    pub repo: String,
    /// Git Branch of the theme repository
    pub branch: Option<String>,
}

impl ThemeMeta {
    pub fn get_id(&self) -> ThemeId {
        create_theme_id(&self.repo, self.branch.as_deref())
    }
}

#[derive(Debug, Deserialize)]
pub struct HyprConfig {
    /// Relative path to the Hyprland config directory in the theme repository
    pub location: PathBuf,
    /// Minimum required Hyprland version. Either a semver-string for a tagged release or 'git' for the latest git version.
    // TODO implement version check
    pub minimum_hyprland_version: String,
}

/// Configuration on how to move dot files to their locations
#[derive(Debug, Deserialize)]
pub struct DotsDirectoryConfig {
    /// What to copy relative from the root of the repository
    pub from: PathBuf,

    /// The destination.
    ///
    /// If not specified it will be `to_copy` appended with `~/`
    pub to: Option<String>,

    // TODO: Parse this as Vec<Glob> and err if they are not valid globs
    /// Which dirs and files to ignore. Useful to ignore your own installer for example.
    /// By default `[ ".hyprtheme/", "./*\.[md|]", "LICENSE", ".git*" ]` is always ignored
    /// You can ignore everything with ['**/*'] and only `include` the dirs/files which you want
    pub ignore: Vec<String>,

    // TODO: Parse this as Vec<Glob> and err if they are not valid globs
    /// Regex strings
    pub include: Vec<String>,
}

/// Setup and cleanup script paths. If not set, uses default values
#[derive(Debug, Deserialize)]
pub struct LifeTimeConfig {
    /// Gets run after the theme got installed. Usually to restart changed apps
    /// Default path: .hyprtheme/setup.sh - If found it will run it, even if not specified
    pub setup: Option<String>,

    /// Gets run after the theme got uninstalled. Usually to kill started apps
    /// Default path: .hyprtheme/cleanup.sh - If found it will run it, even if not specified
    pub cleanup: Option<String>,
}

/// Data for an optional extra configurations, like an optional workspaces setup
/// User can install them or not
#[derive(Debug, Deserialize)]
pub struct ExtraConfig {
    /// The name of the extra configuration.
    ///
    /// For example: `workspaces` (theme author also provides his own workspace setup, which might interfer with the users one)
    pub name: String,

    /// Path to the hyprlang `<extra_config>.conf` which will, if selected by the user, sourced by hyprtheme.conf
    pub path: String,

    /// Gets displayed to the user. Describes what this is
    pub description: Option<String>,
}
