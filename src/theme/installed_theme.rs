use super::theme::Theme;
use anyhow::Result;
use std::path::PathBuf;

// Lets just copy the hyprtheme.toml during the install process, too
// then read it out to see which theme and its version is installed.

/// Get the installed theme.
/// It is an Option type as there might be none installed.
///
/// Optionally a hypr config directory can be given to look it up there.
pub fn get(config_dir: Option<&PathBuf>) -> Result<Option<InstalledTheme>> {
    // Parse  < config_dir >/hyprtheme/meta.toml
    // If not found, return none
    // If err returns an error
    // Otherwise returns `InstalledTheme`
}

pub struct InstalledTheme {
    path: PathBuf,
    config: Theme,
}

impl InstalledTheme {
    /// Update the installed theme
    pub fn update(&self) -> Result<&Self> {
        // git pull in theme data dir
        // run install method on updated theme
    }

    /// Uninstall the installed theme
    ///
    /// Consumes `self` as there wont be an installed theme anymore
    pub fn uninstall(self) -> Result<()> {
        // remove hyprtheme.conf sourcing from hyprtheme.conf
        // remove `hyprtheme/` from `hypr` dir
        // run cleanup.sh (which cd though)
    }
}
