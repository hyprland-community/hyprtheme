use super::installed;
use anyhow::Result;
use std::{fs, path::PathBuf};

/// Check if a theme is installed by its repo url.
pub async fn is_theme_installed(repo_url: &str, config_dir: Option<&PathBuf>) -> Result<bool> {
    installed::get(config_dir)
        .await
        .map(|theme_option| theme_option.map_or(false, |theme| theme.config.meta.repo == repo_url))
}

/// Create a source string to be used in the hyprland config file.
///
/// Example: `source=/home/user/.config/hypr/hyprtheme/hyprtheme.conf`
pub fn create_hyrptheme_source_string(hypr_dir: &PathBuf) -> String {
    create_source_string(&PathBuf::from("./hyprtheme/hyprtheme.conf"), hypr_dir)
}

/// Create a source string for `hyprlang` files.
///
/// Used to source hyprtheme files in `hypr.conf`
pub fn create_source_string(file_path: &PathBuf, hypr_dir: &PathBuf) -> String {
    "source=".to_string() + &hypr_dir.join(file_path).to_string_lossy()
}

/// A branded string type to prevent passing any string where a theme id is expected.
#[derive(Hash, PartialEq, Eq)]
pub struct ThemeId(String);

pub fn create_theme_id(repo: &str, branch: Option<&str>) -> ThemeId {
    ThemeId(format!("{}{}", repo, branch.unwrap_or_else(|| "no_branch")))
}

/// Prepend text to a file.
/// Dont forget to add a newline if necessary
pub fn prepend_to_file(file_path: &PathBuf, contents: &str) -> Result<()> {
    let new_contents = format!("{}{}", contents, fs::read_to_string(file_path)?);

    fs::write(file_path, new_contents)?;

    Ok(())
}
