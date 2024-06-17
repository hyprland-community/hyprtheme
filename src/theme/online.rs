use super::saved::{self, SavedTheme};
use crate::consts::DEFAULT_DOWNLOAD_PATH;
use anyhow::{Context, Result};
use expanduser::expanduser;
use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;
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

    println!("Whut is this dir_name: {}", dir_name.clone());

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

    let themes_dir_string = &themes_dir.to_str().context(format!(
        "Download path {} contains non-unicode characters.",
        &themes_dir.display()
    ))?;
    let theme_dir = themes_dir.join(&dir_name);

    println!("Whut is this themes_dir: {}", themes_dir_string);

    let clone_cmd = format!(
        "mkdir -p {} && git clone --depth 1 {} {} {}",
        themes_dir_string,
        branch
            .map(|branch_name| "--branch ".to_owned() + branch_name)
            .unwrap_or("".to_owned()),
        git_url,
        &dir_name
    );

    println!("Whut is this clone_cmd: {}", clone_cmd);

    std::process::Command::new("sh")
        .arg("-c")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .current_dir(&themes_dir)
        .arg(&clone_cmd)
        .output()?;

    // parse hyprtheme.toml
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
