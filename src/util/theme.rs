use serde::{Deserialize, Serialize};
use crate::util::ansi::{red, green, black, reset, yellow, bold};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Themes {
    pub themes: Vec<Theme>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub repo: String,
    pub branch: String,
    pub config: String,
    pub desc: String,
    pub images: Vec<String>,
    pub _installed: Option<bool>
}

impl Theme {
    pub fn get_author(&self) -> String {
        let mut split = self.repo.split('/').collect::<Vec<&str>>();
        split.reverse();
        split[1].to_string()
    }

    pub async fn fetch_preview(&self) -> Result<Vec<u8>, String> {
        if self.images.len() == 0 {
            return Err("No preview images found".to_string());
        }
        match reqwest::get(&self.images[0]).await {
            Ok(res) => match res.bytes().await {
                Ok(bytes) => Ok(bytes.to_vec()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

// display
impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut images = String::new();
        for image in &self.images {
            images.push_str(&format!("{}\n", image));
        }
        write!(
            f,
            "{}{}{}{}",
            reset(),match self._installed {
                Some(true) => green(false)+&bold()+"● ",
                _ => String::from("○ "),
            },self.name,reset()
        )
    }
}
