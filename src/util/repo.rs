use std::{path::PathBuf, time::Duration};

use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};

use crate::util::theme::{Themes, Theme};

pub fn theme_installed(theme_name: &str, theme_dir: &PathBuf) -> bool {
    let dir = theme_dir.join(theme_name);
    if dir.exists() {
        return true;
    }

    let dir = theme_dir.join(theme_name.to_lowercase());
    if dir.exists() {
        return true;
    }

    return false
}

pub async fn fetch_themes(theme_dir: &PathBuf, file_url:Option<&str>) -> Result<Themes, String> {
    // fetch with progressbar
    let client = Client::new();
    let url = file_url.unwrap_or("https://github.com/hyprland-community/theme-repo/blob/main/themes.json?raw=true");
    
    let progress_bar = ProgressBar::new_spinner();

    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}").unwrap().tick_chars("🌑🌒🌓🌔🌕🌖🌗🌘|")
    );

    progress_bar.enable_steady_tick(Duration::from_millis(50));

    progress_bar.set_message("Fetching themes");

    match client.get(url).send().await {
        Ok(res) => match res.text().await {
            Ok(text) => {
                
                match serde_json::from_str::<Themes>(&text) {
                    Ok(mut themes) => {
                         for theme in &mut themes.themes {
                              theme._installed = Some(theme_installed(&theme.name, &theme_dir));
                         }
                         progress_bar.finish_with_message(format!("Fetched {} themes", &themes.themes.len()));
                         return Ok(themes)
                    },
                    Err(e) => Err(e.to_string()),
                }
    
            },
            Err(e) => {
                progress_bar.finish_with_message("Failed to parse response");
                return Err(e.to_string());
            },
        },
        Err(e) => {
            progress_bar.finish_with_message("Failed to fetch themes");
            return Err(e.to_string());
        },
    }
}

pub async fn find_theme(theme_name: &str, theme_dir: &PathBuf) -> Result<Theme, String> {
    let themes = match fetch_themes(theme_dir,None).await {
        Ok(themes) => themes,
        Err(e) => return Err(e),
    };
    for theme in themes.themes {
        if theme.name.to_lowercase() == theme_name.to_lowercase() {
            return Ok(theme);
        }
    }
    Err("Theme not found".to_string())
}

