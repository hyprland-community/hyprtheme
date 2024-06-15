use std::{path::PathBuf, time::Duration};

use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

// use crate::util::theme::{Themes, Theme};

use crate::modules::{installed::{Installed, Partial}, module::{installed_theme, partial_theme, Module}, theme::Theme};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawThemes {
    pub themes: Vec<RawTheme>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawTheme {
    pub name: String,
    pub repo: String,
    pub branch: String,
    pub config: String,
    pub desc: String,
    pub images: Vec<String>,
    pub _installed: Option<bool>
}




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

pub async fn installed_themes(theme_dir: &PathBuf) -> Vec<Module<Theme<Installed>>> {
    let mut themes = Vec::new();
    for entry in std::fs::read_dir(&theme_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let theme_name = path.file_name().unwrap().to_str().unwrap().to_string();
            themes.push(installed_theme(
                theme_name.clone(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                Vec::new(),
                "".to_string(),
                theme_dir.clone(),
            ));
        }
    }
    themes
}

pub async fn fetch_themes(theme_dir: &PathBuf, file_url:Option<&str>) -> Result<Vec<Module<Theme<Partial>>>, String> {
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
                
                match serde_json::from_str::<RawThemes>(&text) {
                    Ok(themes) => {
                        progress_bar.finish_with_message(format!("Fetched {} themes", &themes.themes.len()));

                        let mut theme_vec = Vec::new();
                        for theme in themes.themes {
                            theme_vec.push(partial_theme(
                                theme.name,
                                theme.repo,
                                theme.branch,
                                theme.desc,
                                theme.images,
                                theme.config,
                                theme_dir.clone(),
                            ));
                        }

                        return Ok(theme_vec);
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

pub async fn find_theme(theme_name: &str, theme_dir: &PathBuf) -> Result<Module<Theme<Partial>>, String> {
    let themes = match fetch_themes(theme_dir,None).await {
        Ok(themes) => themes,
        Err(e) => return Err(e),
    };
    for theme in themes {
        if theme.name.to_lowercase() == theme_name.to_lowercase() {
            return Ok(theme);
        }
    }
    Err("Theme not found".to_string())
}

