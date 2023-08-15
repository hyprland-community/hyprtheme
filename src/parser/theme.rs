use std::fs;
use std::path::{Path, PathBuf};

use toml::Value;

#[derive(Debug, Clone)]
pub struct Kill {
    pub exclude_bar: Vec<String>,
    pub exclude_wallpaper: Vec<String>,
}

impl Kill {
    pub fn from_toml(config: Value) -> Result<Kill, String> {
        let mut kill = Kill {
            exclude_bar: Vec::new(),
            exclude_wallpaper: Vec::new(),
        };

        match config.get("kill") {
            Some(kill_conf) => {
                if let Some(exclude_bar) = kill_conf.get("exclude_bar") {
                    match exclude_bar.as_array() {
                        Some(exclude_bar) => {
                            for bar in exclude_bar {
                                kill.exclude_bar.push(match bar.as_str() {
                                    Some(bar) => bar.to_string(),
                                    None => {
                                        return Err("[Kill] exclude_bar is not a string".to_string())
                                    }
                                })
                            }
                        }
                        None => return Err("[Kill] exclude_bar is not an array".to_string()),
                    }
                };
                if let Some(exclude_wallpaper) = kill_conf.get("exclude_wallpaper") {
                    match exclude_wallpaper.as_array() {
                        Some(exclude_wallpaper) => {
                            for wallpaper in exclude_wallpaper {
                                kill.exclude_wallpaper.push(match wallpaper.as_str() {
                                    Some(wallpaper) => wallpaper.to_string(),
                                    None => {
                                        return Err(
                                            "[Kill] exclude_wallpaper is not a string".to_string()
                                        )
                                    }
                                })
                            }
                        }
                        None => return Err("[Kill] exclude_wallpaper is not an array".to_string()),
                    }
                };
                Ok(kill)
            }
            None => Ok(kill),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Script {
    pub load: PathBuf,
    pub cleanup: PathBuf,
}
impl Script {
    pub fn from_toml(config: Value) -> Script {
        let mut script = Script {
            load: PathBuf::new(),
            cleanup: PathBuf::new(),
        };
        let script_block = match config.get("script") {
            Some(script_block) => script_block,
            None => return script,
        };
        if let Some(load) = script_block.get("load") {
            script.load = PathBuf::from(load.as_str().expect("load is not a string"));
        }
        if let Some(cleanup) = script_block.get("cleanup") {
            script.cleanup = PathBuf::from(cleanup.as_str().expect("cleanup is not a string"));
        }
        script
    }
}

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
    pub _script: Script,
    pub _kill: Kill,
    pub _repo: Option<String>,
    pub _path: PathBuf,
}

impl Theme {
    pub fn parse_config(mut raw: String, path: PathBuf) -> Result<Theme, String> {
        raw = raw.replace("./", "");
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
                            Some(subtheme) => subtheme,
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
                    .join(conf)
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

        let kill = match Kill::from_toml(config.clone()) {
            Ok(kill) => kill,
            Err(e) => return Err(e),
        };

        let mut script = Script::from_toml(config.clone());
        let script = {
            if script.load.is_relative() {
                script.load = path.parent().unwrap().join(script.load);
            }
            if script.cleanup.is_relative() {
                script.cleanup = path.parent().unwrap().join(script.cleanup);
            }
            script
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
            depends,
            default_subtheme,
            _script: script,
            _kill: kill,
            _repo: None,
            _path: path.parent().unwrap().to_path_buf(),
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
