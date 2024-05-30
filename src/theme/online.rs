use std::path::{Path, PathBuf};

use anyhow::Result;
use reqwest::Client;

use serde::Deserialize;

use crate::consts::DEFAULT_DOWNLOAD_PATH;

use super::saved::SavedTheme;

// Contains the code to interact with featured themes
// in the Hyprtheme repo

// What should an online theme be able to do?
// It should be downloadable

/// A fetched theme featured on Hyprtheme
#[derive(Deserialize, Debug)]
pub struct OnlineTheme {
    name: String,
    repo: String,
    /// Branch of the repo. Optional, will otherwise just git clone without a specified branch
    branch: Option<String>,
    // Todo figure out if this is nessesary, as it creates more maintenance burden
    // As of right now, it is not
    config: String,
    desc: String,
    images: Vec<String>,
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
