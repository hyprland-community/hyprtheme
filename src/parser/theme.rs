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
    pub fn parse_config(raw: String, path: PathBuf) -> Result<Theme, String> {
        let config = match raw.parse::<toml::Value>() {
            Ok(config) => config,
            Err(e) => return Err(format!("Error parsing theme config: {} : {}", e, raw)),
        };

        let mut subthemes: Vec<Theme> = Vec::new();

        let theme_info = match config.get("theme") {
            Some(theme_info) => theme_info,
            None => {
                return Err(format!(
                    "Theme file {} is missing theme info",
                    &path.display()
                ))
            }
        };

        if let Some(__subthemes) = theme_info.get("subthemes") {
            for subtheme in match __subthemes.as_array() {
                Some(subthemes) => subthemes,
                None => {
                    return Err(format!(
                        "Theme file {} has invalid subthemes(not an array)",
                        &path.display()
                    ))
                }
            } {
                let subtheme_path = {
                    match path.parent() {
                        Some(parent) => parent.join(match subtheme.as_str() {
                            Some(subtheme) => subtheme.replace("./", ""),
                            None => {
                                return Err(format!(
                                    "Theme file {} has invalid subtheme(not a string)",
                                    &path.display()
                                ))
                            }
                        }),
                        None => {
                            return Err(format!(
                                "Theme file {} has invalid subtheme(not a string)",
                                &path.display()
                            ))
                        }
                    }
                };
                match Theme::from_file(subtheme_path) {
                    Ok(subtheme) => subthemes.push(subtheme),
                    Err(e) => return Err(e),
                }
            }
        };

        let config_path = match theme_info.get("config") {
            Some(conf) => {
                let conf = conf.as_str().expect("subtheme is not a string");
                if conf.is_empty() && subthemes.is_empty() {
                    return Err(format!(
                        "Theme file {} is missing conf path",
                        path.display()
                    ));
                }
                path.parent()
                    .expect("theme has no parent directory")
                    .join(conf.replace("./", ""))
            }
            None => {
                if subthemes.is_empty() {
                    return Err(format!(
                        "Theme file {} is missing conf path",
                        path.display()
                    ));
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
                for dep in match depends.as_array() {
                    Some(depends) => depends,
                    None => {
                        return Err(format!(
                            "Theme file {} has invalid depends(not an array)",
                            &path.display()
                        ))
                    }
                } {
                    deps.push(match dep.as_str() {
                        Some(dep) => dep.to_string(),
                        None => {
                            return Err(format!(
                                "Theme file {} has invalid depend(not a string)",
                                &path.display()
                            ))
                        }
                    });
                }
                deps
            }
            None => Vec::new(),
        };

        Ok(Theme {
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
        })
    }

    pub fn from_file(path: PathBuf) -> Result<Theme, String> {
        let raw = match fs::read_to_string(&path) {
            Ok(f) => f,
            Err(e) => return Err(format!("error reading file: {}", e)),
        };
        Theme::parse_config(raw, path)
    }

    pub fn from_string(raw: String, path: PathBuf) -> Result<Theme, String> {
        Theme::parse_config(raw, path)
    }
}
