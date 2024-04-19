use std::path::PathBuf;

use crate::util::ansi::{bold, green, reset};
use serde::{Deserialize, Serialize};

use expanduser::expanduser;

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
    pub _installed: Option<bool>,
}

impl Theme {
    pub fn install(&self, install_dir: Option<PathBuf>) -> Result<(), String> {
        let install_dir = install_dir.unwrap_or(expanduser("~/.config/hypr/themes").unwrap());

        //standardize theme name
        let theme_name = self.name.to_lowercase().replace(" ", "_");

        let theme_dir = install_dir.join(&theme_name);

        // check if theme is already installed
        if theme_dir.exists() {
            return Err(format!("Theme {} is already installed", &self.name));
        }

        println!(
            "Installing theme {} to {}\n",
            &self.name,
            theme_dir.to_str().unwrap()
        );
        // clone repo
        let clone_cmd = format!(
            "git clone --depth 1 --branch {} {} {}",
            &self.branch,
            &self.repo,
            theme_dir.to_str().unwrap()
        );

        match std::process::Command::new("sh")
            .arg("-c")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg(&clone_cmd)
            .output()
        {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn uninstall(&self, install_dir: Option<PathBuf>) -> Result<(), String> {
        let install_dir = install_dir.unwrap_or(expanduser("~/.config/hypr/themes").unwrap());

        //standardize theme name
        let theme_name = self.name.to_lowercase().replace(" ", "_");

        let theme_dir = install_dir.join(&theme_name);

        // check if theme is already installed
        if !theme_dir.exists() {
            return Err(format!("Theme {} is not installed", &self.name));
        }

        println!(
            "Uninstalling theme {} from {}",
            &self.name,
            theme_dir.to_str().unwrap()
        );

        // delete dir
        match std::fs::remove_dir_all(theme_dir) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn update(&self, install_dir: Option<PathBuf>) -> Result<(), String> {
        let install_dir = install_dir.unwrap_or(expanduser("~/.config/hypr/themes").unwrap());

        //standardize theme name
        let theme_name = self.name.to_lowercase().replace(" ", "_");

        let theme_dir = install_dir.join(&theme_name);

        // check if theme is already installed
        if !theme_dir.exists() {
            return Err(format!("Theme {} is not installed", &self.name));
        }

        println!(
            "Updating theme {} in {}",
            &self.name,
            theme_dir.to_str().unwrap()
        );

        // delete dir
        match std::process::Command::new("sh")
            .arg("-c")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(&theme_dir)
            .arg("git pull")
            .output()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    // pub fn get_author(&self) -> String {
    //     let mut split = self.repo.split('/').collect::<Vec<&str>>();
    //     split.reverse();
    //     split[1].to_string()
    // }

    // pub async fn fetch_preview(&self) -> Result<Vec<u8>, String> {
    //     if self.images.len() == 0 {
    //         return Err("No preview images found".to_string());
    //     }
    //     match reqwest::get(&self.images[0]).await {
    //         Ok(res) => match res.bytes().await {
    //             Ok(bytes) => Ok(bytes.to_vec()),
    //             Err(e) => Err(e.to_string()),
    //         },
    //         Err(e) => Err(e.to_string()),
    //     }
    // }
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
            reset(),
            match self._installed {
                Some(true) => green(false) + &bold() + "● ",
                _ => String::from("○ "),
            },
            self.name,
            reset()
        )
    }
}
