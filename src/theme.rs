use std::fs;
use std::path::{Path, PathBuf};

use toml;

use crate::config::{Component,Config};

#[derive(Debug,Clone)]
pub struct Theme {
    pub conf: PathBuf,
    pub name: String,
    pub desc: String,
    pub author: String,
    pub git: String,
    pub version: String,
    pub subthemes: Vec<Theme>,
    pub default_subtheme: String,
    pub components: Vec<Component>,
    pub hyprpaper: PathBuf,
    pub path: PathBuf,
}

impl Theme {
    pub fn config_from_file(path: PathBuf) -> Config{
        let theme = Theme::from_file(path);
        Config::from_theme(theme)
    }

    pub fn from_file(path: PathBuf) -> Theme {
        let raw = fs::read_to_string(&path).expect("Unable to read file");
        let config = raw.parse::<toml::Value>().expect("Unable to parse toml");

        let mut subthemes: Vec<Theme> = Vec::new();
        let mut components: Vec<Component> = Vec::new();

        let theme_info = match config.get("theme") {
            Some(theme_info) => theme_info,
            None => panic!("Theme file {} is missing theme info", &path.display()),
        };

        match theme_info.get("subthemes") {
            Some(__subthemes) => {
                for subtheme in __subthemes.as_array().expect("subthemes is not an array") {
                    let subtheme_path = {
                        path.parent().expect("theme has no parent directory").join(
                            subtheme
                                .as_str()
                                .expect("subtheme is not a string")
                                .replace("./", ""),
                        )
                    };
                    subthemes.push(Theme::from_file(subtheme_path));
                }
            }
            None => {}
        };

        let conf = match theme_info.get("conf") {
            Some(conf) => {
                let conf = conf.as_str().expect("subtheme is not a string");
                if conf == "" && subthemes.len() < 1 {
                    panic!("Theme file {} is missing conf path", path.display())
                }
                path.parent()
                    .expect("theme has no parent directory")
                    .join(conf.replace("./", ""))
            }
            None => {
                if subthemes.len() < 1 {
                    panic!("Theme file {} is missing conf path", path.display())
                } else {
                    Path::new("").to_path_buf()
                }
            }
        };

        let hyprpaper = match theme_info.get("hyprpaper") {
            Some(hyprpaper) => {
                let hyprpaper = hyprpaper.as_str().unwrap();
                if hyprpaper == "" {
                    Path::new("").to_path_buf()
                } else {
                    path.parent()
                        .expect("theme has no parent directory")
                        .join(hyprpaper.replace("./", ""))
                }
            }
            None => Path::new("").to_path_buf(),
        };

        let name = match theme_info.get("name") {
            Some(name) => name.as_str().unwrap().to_string(),
            None => String::new(),
        };

        let desc = match theme_info.get("desc") {
            Some(desc) => desc.as_str().unwrap().to_string(),
            None => String::new(),
        };

        let author = match theme_info.get("author") {
            Some(author) => author.as_str().unwrap().to_string(),
            None => String::new(),
        };

        let git = match theme_info.get("git") {
            Some(git) => git.as_str().unwrap().to_string(),
            None => String::new(),
        };

        let version = match theme_info.get("version") {
            Some(version) => version.as_str().unwrap().to_string(),
            None => String::new(),
        };

        let default_subtheme = match theme_info.get("default_subtheme") {
            Some(default_subtheme) => default_subtheme.as_str().unwrap().to_string(),
            None => String::new(),
        };

        Theme {
            name,
            desc,
            conf,
            author,
            git,
            version,
            subthemes,
            components,
            path,
            default_subtheme,
            hyprpaper,
        }
    }
}
