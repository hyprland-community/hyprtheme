use super::saved::{self, SavedTheme};
use crate::consts::DEFAULT_DOWNLOAD_PATH;
use anyhow::{anyhow, Context, Result};
use expanduser::expanduser;
use reqwest::Client;
use serde::Deserialize;
use std::{fs, path::PathBuf};
use url::Url;

// Contains the code to interact with featured themes
// in the Hyprtheme repo

// What should an online theme be able to do?
// It should be downloadable

/// A fetched theme featured on Hyprtheme
#[derive(Deserialize, Debug)]
pub struct OnlineTheme {
    pub name: String,
    pub repo: String,
    /// Branch of the repo. Optional, will otherwise just git clone without a specified branch
    pub branch: Option<String>,
    // Config is currently unused
    pub config: String,
    pub desc: String,
    pub images: Vec<String>,
}

pub async fn fetch_themes(themes_json_url: Option<&str>) -> Result<Vec<OnlineTheme>> {
    let client = Client::new();
    let url = themes_json_url.unwrap_or(
        "https://github.com/hyprland-community/theme-repo/blob/main/themes.json?raw=true",
    );

    #[derive(Deserialize, Debug)]
    struct ThemesData {
        themes: Vec<OnlineTheme>,
    }

    Ok(
        serde_json::from_str::<ThemesData>(client.get(url).send().await?.text().await?.as_str())?
            .themes,
    )
}

/// Lookup featured theme by name
pub async fn find_featured(theme_name: &str) -> Result<Option<OnlineTheme>> {
    let found_theme = fetch_themes(None)
        .await?
        .into_iter()
        .find(|theme| theme.name.to_lowercase() == theme_name.to_lowercase());

    Ok(found_theme)
}

/// Download a theme from a repo into the data dir and parse it
pub async fn download(
    git_url: &String,
    branch: Option<&str>,
    data_dir: Option<&PathBuf>,
) -> Result<SavedTheme> {
    let url = Url::parse(&git_url).context("Invalid git URL passed")?;
    // To avoid name conflicts, we use the host + the repo path, as there could be themes with the same author and theme name on different Git hosts.
    let dir_name = format!(
        "{}{}{}",
        url.host()
            .map(|host| host.to_string())
            .unwrap_or("".to_string()),
        url.path(),
        branch.map(|b| format!(".{}", b)).unwrap_or("".to_string())
    )
    .replace("/", ".")
    .replace("\\", ".");

    // clone repo
    let themes_dir = expanduser(
        data_dir
            .map(|path| {
                path.to_str()
                    .expect("Data directory contains invalid characters")
            })
            .unwrap_or(DEFAULT_DOWNLOAD_PATH),
    )
    .context(format!(
        "Failed to expand default download path: {}",
        DEFAULT_DOWNLOAD_PATH
    ))?
    .join("themes/");

    fs::create_dir_all(&themes_dir).context(format!(
        "Failed to create themes directory to download theme: {}",
        &themes_dir.display()
    ))?;

    let clone_cmd = format!(
        "git clone --depth 1 {} {} {}",
        branch
            .map(|branch_name| "--branch ".to_owned() + branch_name)
            .unwrap_or("".to_owned()),
        git_url,
        &dir_name
    );

    let clone_operation = std::process::Command::new("sh")
        .arg("-c")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&clone_cmd)
        .current_dir(&themes_dir)
        .output()
        .context(format!("Failed to execute clone. Command: {}", &clone_cmd))?;

    if !clone_operation.status.success() {
        return Err(anyhow!("Error when cloning theme repository"));
    }
    // parse hyprtheme.toml
    let theme_dir = themes_dir.join(&dir_name);
    saved::from_directory(&theme_dir).await
}

// impl OnlineTheme {
// pub async fn download(&self, data_dir: Option<&PathBuf>) -> Result<SavedTheme> {
//     download(&self.repo, self.branch.as_deref(), data_dir).await
// }

// pub async fn is_installed(&self, config_dir: Option<&PathBuf>) -> Result<bool> {
//     is_theme_installed(&self.get_id(), config_dir).await
// }

// /// Compute the id of the theme. Used to compare it with saved themes
// pub fn get_id(&self) -> ThemeId {
//     create_theme_id(&self.repo, self.branch.as_deref())
// }

// pub async fn is_saved(&self, data_dir: Option<&PathBuf>) -> Result<bool> {
//     Ok(saved::find_saved(&self.get_id(), data_dir)
//         .await
//         .context("Failure when locating saved theme")?
//         .is_some())
// }
// }
