use crate::consts::DEFAULT_HYPR_CONFIG_PATH;

use super::{
    helper::create_hyrptheme_source_string,
    online,
    saved::{self, SavedTheme, ThemeMeta},
};
use anyhow::{anyhow, Context, Result};
use expanduser::expanduser;
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

// Lets just copy the hyprtheme.toml during the install process, too
// then read it out to see which theme and its version is installed.

/// Get the installed theme.
/// It is an Option type as there might be none installed.
///
/// Optionally a hypr config directory can be given to look it up there.
pub async fn get(config_dir: Option<&PathBuf>) -> Result<Option<InstalledTheme>> {
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
    pub async fn update(&self, data_dir: Option<&PathBuf>) -> Result<Self> {
        let saved_result = saved::find_saved(&self.meta.name, data_dir).await.context(
            "Error looking up stored repository of the installed theme. Try reinstalling it again.",
        )?;

        // download theme if it is not saved anymore
        // nessecary if someone downloaded dots which use Hyprtheme and the data dir is not in them.
        // Also the data dir should not be source controlled as they can be big and there can be many themes
        let saved = match saved_result {
            Some(ok) => ok,
            None => {
                online::download(&self.meta.repo, self.meta.branch.as_deref(), data_dir).await?
            }
        };

        saved.update()?.install(Some(&self.path)).await
    }

    /// Uninstall the installed theme
    ///
    /// Consumes `self` as there wont be an installed theme anymore
    ///
    /// - hypr_dir: Path to the hypr config directory
    pub fn uninstall(self, hypr_dir: Option<&PathBuf>) -> Result<()> {
        let default_dir = &expanduser(DEFAULT_HYPR_CONFIG_PATH)?;
        let hypr_dir = hypr_dir.unwrap_or(default_dir);
        let hyprtheme_dir = hypr_dir.join("./hyprtheme/");
        let hyprland_config_path = hypr_dir.join("./hyprland.conf");

        // remove `hyprtheme/` from `hypr` dir
        fs::remove_dir_all(hyprtheme_dir)
            .context("Failed to remove /hyprtheme in hypr config directory")?;

        // Remove source string from hyprland.conf
        let hyprtheme_source_str = create_hyrptheme_source_string(hypr_dir);
        // Read out hyprland.conf via std::fs::read_to_string("list.txt").unwrap();
        // Check if hyprtheme.conf is sourced in hyprland.conf, if not, source it
        let hyprland_config = fs::read_to_string(&hyprland_config_path)
            .context("Failed to read out hyprland.conf")?;
        let is_already_sourced = hyprland_config.contains(&hyprtheme_source_str);

        if is_already_sourced {
            let config_str = fs::read_to_string(&hyprland_config)?
                .replace(&hyprtheme_source_str.to_string(), "");

            fs::write(hyprland_config_path, config_str)?;
        }

        Ok(())

        // TODO
        // Run cleanup.sh
    }
}
