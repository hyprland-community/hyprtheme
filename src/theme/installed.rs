use super::saved::{self, SavedTheme, ThemeMeta};
use anyhow::{anyhow, Context, Result};
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
    /// Path of the hypr config directory where this theme is installed in
    path: PathBuf,
    /// The config which got copied over to `.config/hypr/hyprtheme/theme.toml`
    // We only save the meta, as this is not a `SavedTheme`, thus shouldn't have the same methods,
    // but we might want to query data about it
    pub meta: ThemeMeta,
}

impl InstalledTheme {
    /// Update the installed theme
    ///
    /// Consumes self, as this will overwrite the currently installed theme
    /// with the updated version
    pub async fn update(self) -> Result<Self> {
        let saved = saved::get_saved(&self.meta.name)
            .context("Error looking up stored repository of the installed theme.")?
            .ok_or(anyhow!(
                "Could not find saved repository of the installed theme in the data directory."
            ))?;

        // TODO download theme if it is not saved anymore
        // nessecary if someone downloaded dots which use Hyprtheme and the data dir is not in them
        // The data dir should not be source controlled as they can be big and there can be many

        saved.update()?.install(Some(&self.path)).await

        // TODO run setup.sh
    }

    /// Uninstall the installed theme
    ///
    /// Consumes `self` as there wont be an installed theme anymore
    pub fn uninstall(self) -> Result<()> {
        // TODO
        // remove hyprtheme.conf sourcing from hyprtheme.conf
        // remove `hyprtheme/` from `hypr` dir
        // run cleanup.sh (which cd though)
    }
}
