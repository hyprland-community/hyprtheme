use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use expanduser::expanduser;
use reqwest::Client;

use serde::Deserialize;
use url::Url;

use crate::consts::DEFAULT_DOWNLOAD_PATH;

use super::saved::SavedTheme;

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
    // Todo figure out if this is nessesary, as it creates more maintenance burden
    // As of right now, it is not
    pub config: String,
    pub desc: String,
    pub images: Vec<String>,
}

impl OnlineTheme {
    pub fn download(&self) -> Result<SavedTheme> {
        // TODO
        // reuse the function from `saved` theme?
        // Maybe I just use a standalone download function instead and share it
    }

    fn is_installed(&self, theme_dir: Option<&PathBuf>) -> Result<bool> {
        let dir: PathBuf = theme_dir
            .unwrap_or(&expanduser::expanduser(DEFAULT_DOWNLOAD_PATH)?)
            .join(&self.name);

        // TODO this is a FS call, so why is it not a result?
        Ok(dir.exists())
    }

    // TODO maybe
    // Open the theme in the browser
    // fn open_repo(&self) -> () {}
}

pub async fn fetch_themes(themes_json_url: Option<&str>) -> Result<Vec<OnlineTheme>> {
    let client = Client::new();
    let url = themes_json_url.unwrap_or(
        "https://github.com/hyprland-community/theme-repo/blob/main/themes.json?raw=true",
    );

    // let progress_bar = ProgressBar::new_spinner();
    // progress_bar.set_style(
    //     ProgressStyle::default_spinner()
    //         .template("{spinner} {msg}")
    //         .unwrap()
    //         .tick_chars("🌑🌒🌓🌔🌕🌖🌗🌘|"),
    // );
    // progress_bar.enable_steady_tick(Duration::from_millis(50));
    // progress_bar.set_message("Fetching themes");

    Ok(serde_json::from_str::<Vec<OnlineTheme>>(
        client.get(url).send().await?.text().await?.as_str(),
    )?)
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
    branch: Option<&String>,
    save_dir: Option<&PathBuf>,
) -> Result<SavedTheme> {
    // We need to first download the repo, before we can parse its config
    let url = Url::parse(&git_url).context("Invalid URL passed")?;
    let dir_name = url
        .path()
        .split("/")
        .last()
        .expect("Invalid Git URL passed");

    // clone repo
    let clone_path = expanduser(
        save_dir
            .map(|path| path.to_str().unwrap())
            .unwrap_or(DEFAULT_DOWNLOAD_PATH),
    )
    .context(format!(
        "Failed to expand default download path: {}",
        DEFAULT_DOWNLOAD_PATH
    ))?
    .join(dir_name);
    let clone_path_string = &clone_path
        .to_str()
        .context(format!("Download path contains non-unicode characters."))?;

    let clone_cmd = format!(
        "mkdir -p {} && cd {} && git clone --depth 1 {} {}",
        clone_path_string,
        clone_path_string,
        branch
            .map(|branch_name| "--branch ".to_owned() + branch_name)
            .unwrap_or("".to_owned()),
        git_url
    );

    std::process::Command::new("sh")
        .arg("-c")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&clone_cmd)
        .output()?;

    // parse hyprtheme.toml
    SavedTheme::from_directory(&clone_path).await
}
