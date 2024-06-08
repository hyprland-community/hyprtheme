use super::installed;
use anyhow::Result;
use std::path::PathBuf;

/// Check if a theme is installed by its repo url.
pub async fn is_theme_installed(repo_url: &str, config_dir: Option<&PathBuf>) -> Result<bool> {
    installed::get(config_dir)
        .await
        .map(|theme_option| theme_option.map_or(false, |theme| theme.meta.repo == repo_url))
}

/// Create a source string to be used in the hyprland config file.
///
/// Example: `source=/home/user/.config/hypr/hyprtheme/hyprtheme.conf`
pub fn create_hyrptheme_source_string(hypr_dir: &PathBuf) -> String {
    "source=".to_string()
        + &hypr_dir
            .join("./hyprtheme/hyprtheme.conf")
            .to_string_lossy()
}

#[derive(Hash, PartialEq, Eq)]
pub struct ThemeId(String);

pub fn create_theme_id(repo: &str, branch: Option<&str>) -> ThemeId {
    ThemeId(format!("{}{}", repo, branch.unwrap_or_else(|| "no_branch")))
}
