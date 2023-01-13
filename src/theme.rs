use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Theme {
    pub config_path: PathBuf,
    pub name: String,
    pub desc: String,
    pub author: String,
    pub git: String,
    pub version: String,
    pub subthemes: Vec<Theme>,
    pub default_subtheme: String,
    pub depends: Vec<String>,
    pub _repo: Option<String>,
    pub _path: PathBuf,
}

impl Theme {
    pub fn parse_config(raw: String, path: PathBuf) -> Theme {
        let config = raw.parse::<toml::Value>().expect("Unable to parse toml");

        let mut subthemes: Vec<Theme> = Vec::new();

        let theme_info = match config.get("theme") {
            Some(theme_info) => theme_info,
            None => panic!("Theme file {} is missing theme info", &path.display()),
        };

        if let Some(__subthemes) = theme_info.get("subthemes") {
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
        };

        let config_path = match theme_info.get("config") {
            Some(conf) => {
                let conf = conf.as_str().expect("subtheme is not a string");
                if conf.is_empty() && subthemes.is_empty() {
                    panic!("Theme file {} is missing conf path", path.display())
                }
                path.parent()
                    .expect("theme has no parent directory")
                    .join(conf.replace("./", ""))
            }
            None => {
                if subthemes.is_empty() {
                    panic!("Theme file {} is missing conf path", path.display())
                } else {
                    Path::new("").to_path_buf()
                }
            }
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

        let depends = match theme_info.get("depends") {
            Some(depends) => {
                let mut deps: Vec<String> = Vec::new();
                for dep in depends.as_array().expect("depends is not an array") {
                    deps.push(dep.as_str().expect("dep is not a string").to_string());
                }
                deps
            },
            None => Vec::new(),
        };

        Theme {
            name,
            desc,
            config_path,
            author,
            git,
            version,
            subthemes,
            _repo: None,
            _path: path,
            depends,
            default_subtheme,
        }
    }

    pub fn from_file(path: PathBuf) -> Theme {
        let raw = fs::read_to_string(&path).expect("Unable to read file");
        Theme::parse_config(raw, path)
    }

    pub fn from_string(raw: String, path: PathBuf) -> Theme {
        Theme::parse_config(raw, path)
    }
}
