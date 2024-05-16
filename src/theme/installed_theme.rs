use super::theme::Theme;
use anyhow::Result;
use std::path::PathBuf;

// Lets just copy the hyprtheme.toml during the install process, too
// then read it out to see which theme and its version is installed.

/// Retrieves the installed theme. Optionally a hypr config path can be given to look it up there.
pub fn get(config_path: Option<&PathBuf>) -> Result<Option<InstalledTheme>> {}

pub struct InstalledTheme {
    path: PathBuf,
    config: Theme,
}

impl InstalledTheme {
    /// Get the installed theme.
    /// It is an Option type as there might be none installed.

    /// Update the installed theme
    pub fn update(&self) -> Result<&Self> {}

    /// Uninstall the installed theme
    pub fn uninstall(self) -> Result<()> {}
}
