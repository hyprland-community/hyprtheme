use reqwest::get;

use crate::util::theme::{Themes, Theme};

pub async fn fetch_themes_raw(file_url: Option<&str>) -> Result<String, String> {
    match get(file_url.unwrap_or("https://github.com/hyprland-community/theme-repo/blob/main/themes.json?raw=true")).await {
        Ok(res) => match res.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub async fn fetch_themes() -> Result<Themes, String> {
   match fetch_themes_raw(None).await {
       Ok(raw) => match serde_json::from_str(&raw) {
           Ok(themes) => Ok(themes),
           Err(e) => Err(e.to_string()),
       },
       Err(e) => Err(e.to_string()),
   }
}

pub async fn find_theme(theme_name: &str) -> Result<Theme, String> {
    let themes = match fetch_themes().await {
        Ok(themes) => themes,
        Err(e) => return Err(e),
    };
    for theme in themes.themes {
        if theme.name == theme_name {
            return Ok(theme);
        }
    }
    Err("Theme not found".to_string())
}

