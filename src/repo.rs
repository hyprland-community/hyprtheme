use reqwest::get;

use crate::util::theme::Themes;

pub async fn fetch_themes_raw(file_url: Option<&str>) -> Result<String, String> {
    match get(file_url.unwrap_or("https://github.com/hyprland-community/theme-repo/blob/main/themes.json?raw=true")).await {
        Ok(res) => match res.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub async fn fetch_themes(file_url: Option<&str>) -> Result<Themes, String> {
   match fetch_themes_raw(file_url).await {
       Ok(raw) => match serde_json::from_str(&raw) {
           Ok(themes) => Ok(themes),
           Err(e) => Err(e.to_string()),
       },
       Err(e) => Err(e.to_string()),
   }
}

