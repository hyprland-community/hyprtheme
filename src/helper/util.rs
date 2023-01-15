use crate::{helper::consts, parser::theme::Theme, render};
use std::{
    fs,
    io::BufReader,
    path::{self, Path, PathBuf},
    process,
};

pub struct ThemeMap {
    pub name: String,
    pub subthemes: Vec<ThemeMap>,
    pub git: Option<String>,
}

impl ThemeMap {
    pub fn from_theme(theme: Theme) -> ThemeMap {
        let mut subthemes: Vec<ThemeMap> = Vec::new();

        for subtheme in theme.subthemes.iter() {
            subthemes.push(ThemeMap::from_theme(subtheme.clone()));
        }
        ThemeMap {
            name: theme.name,
            subthemes,
            git: Some(theme.git),
        }
    }
}

pub fn get_subtheme(theme: &Theme) -> Option<Theme> {
    let subtheme_str = &theme.default_subtheme;

    if !subtheme_str.trim().is_empty() {
        let nest = subtheme_str.split(':').into_iter().collect::<Vec<&str>>();

        let mut subtheme = theme.subthemes.iter().find(|t| t.name == nest[0]).unwrap();

        for n in nest.iter().skip(1) {
            subtheme = subtheme.subthemes.iter().find(|t| t.name == *n).unwrap();
        }

        return Some(subtheme.clone());
    }
    None
}

pub fn find_theme(theme: &str) -> Result<PathBuf, String> {
    // expand user
    let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();

    if theme == "dist" {
        return Err("dist is a reserved theme_name".to_string());
    }

    for entry in path::Path::new(&theme_dir).read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if dir_name == theme {
                if path.join("theme.toml").exists() {
                    return Ok(path.join("theme.toml"));
                } else {
                    return Err(format!("Theme {} is missing theme.toml", theme));
                }
            }
        }
    }
    Err(format!("Theme {} not found", theme))
}

pub fn list_themes() -> Vec<String> {
    let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();
    let mut themes: Vec<String> = Vec::new();

    for entry in theme_dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.file_name().unwrap_or_default() == "dist" {
            continue;
        }
        if path.is_dir() && path.join("theme.toml").exists() {
            themes.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }
    themes
}

pub fn list_themes_deep() -> Result<Vec<ThemeMap>, String> {
    let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();
    let mut themes: Vec<ThemeMap> = Vec::new();

    for entry in theme_dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let theme_path = path.join("theme.toml");
            if theme_path.exists() {
                themes.push(ThemeMap::from_theme(match Theme::from_file(theme_path) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                }));
            }
        }
    }
    Ok(themes)
}

pub struct Repo {}

impl Repo {
    fn get_theme_info(theme_url: &str, branch: Option<&str>) -> Result<Theme, String> {
        let branch = branch.unwrap_or("master");
        let theme_raw = reqwest::blocking::get(format!(
            "{}/blob/{}/theme.toml?raw=true",
            theme_url.to_owned(),
            branch
        ))
        .unwrap()
        .text()
        .unwrap();

        match Theme::from_string(theme_raw, Path::new("/tmp").to_path_buf()) {
            Ok(mut theme) => {
                if theme._repo.is_none() {
                    theme._repo = Some(theme_url.to_owned());
                }
                Ok(theme)
            }
            Err(e) => Err(e),
        }
    }
    pub fn list_themes() -> Result<Vec<Theme>, String> {
        let modules_url =
            "https://raw.githubusercontent.com/hyprland-community/theme-repo/main/.gitmodules";
        let raw = reqwest::blocking::get(modules_url).unwrap().text().unwrap();
        let bytereader = BufReader::new(raw.as_bytes());
        let modules = gitmodules::read_gitmodules(bytereader).unwrap();
        let mut themes = Vec::new();
        for module in modules.iter() {
            let theme_url = module
                .entries()
                .iter()
                .find(|e| e.0 == "url")
                .unwrap()
                .to_owned()
                .1;

            match Repo::get_theme_info(&theme_url, None) {
                Ok(theme) => themes.push(theme),
                Err(e1) => match Repo::get_theme_info(&theme_url, Some("main")) {
                    Ok(theme) => themes.push(theme),
                    Err(e2) => render::warn(format!(
                        "skipping {}, master branch had an error, tried main branch:\n  master: {}\n  main: {}",
                        theme_url, e1, e2
                    ).as_str()),
                },
            }
        }
        Ok(themes)
    }

    pub fn list_themes_deep() -> Result<Vec<ThemeMap>, String> {
        let mut themes: Vec<ThemeMap> = Vec::new();
        for theme in match Repo::list_themes() {
            Ok(themes) => themes,
            Err(e) => return Err(e),
        } {
            themes.push(ThemeMap::from_theme(theme.clone()));
        }
        Ok(themes)
    }

    pub fn install_theme(theme_name: &str) -> Result<PathBuf, String> {
        let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();

        let themes = match Repo::list_themes() {
            Ok(themes) => themes,
            Err(e) => return Err(e),
        };

        let theme = themes.iter().find(|t| t.name == theme_name);

        match theme {
            Some(t) => {
                match process::Command::new("git")
                    .arg("clone")
                    .arg(t._repo.as_ref().unwrap())
                    .arg(&theme_dir.join(&t.name))
                    .output()
                {
                    Ok(_) => {}
                    Err(_) => return Err(format!("Failed to clone theme {}", theme_name)),
                };
                match process::Command::new("chmod")
                    .arg("+x")
                    .arg(&theme_dir.join(&t.name).join("load"))
                    .output()
                {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(format!(
                            "Failed to chmod +x `load` script for theme {}",
                            theme_name
                        ))
                    }
                };
                Ok(theme_dir.join(&t.name))
            }
            None => Err(format!("Theme {} not found", theme_name)),
        }
    }
}

pub struct Util {}

impl Util {
    pub fn kill_all() {
        Util::kill_all_bars(None);
        Util::kill_all_wallpapers(None);
    }

    pub fn kill_all_bars(exclude: Option<Vec<String>>) {
        let pkill = vec!["swaybar", "waybar", "ironbar", "eww"];
        let exclude = exclude.unwrap_or_default();

        for p in pkill.iter() {
            if exclude.contains(&p.to_string()) {
                println!("skipping {}", p);
                continue;
            }
            match process::Command::new("pkill").arg(p).spawn() {
                Ok(_) => println!("killed {}", p),
                Err(_) => println!("{} not running", p),
            }
        }
    }

    pub fn kill_all_wallpapers(exclude: Option<Vec<String>>) {
        let pkill = vec!["swaybg", "wbg", "hyprpaper", "mpvpaper", "swww"];

        let exclude = exclude.unwrap_or_default();

        for p in pkill.iter() {
            if exclude.contains(&p.to_string()) {
                println!("skipping {}", p);
                continue;
            }
            match process::Command::new("pkill").arg(p).spawn() {
                Ok(_) => println!("killed {}", p),
                Err(_) => println!("{} not running", p),
            }
        }
    }

    pub fn create_template(_template_dir: Option<PathBuf>) -> Result<PathBuf, String> {
        let template_dir = match _template_dir {
            Some(_) => _template_dir.unwrap(),
            None => {
                let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();
                theme_dir.join("template")
            }
        };

        if template_dir.exists() && template_dir.is_dir() {
            return Err("Template already exists".to_string());
        }

        fs::create_dir_all(&template_dir).unwrap();

        fs::write(template_dir.join("theme.toml"), consts::T_TOML).unwrap();
        fs::write(template_dir.join("theme.conf"), consts::T_CONF).unwrap();
        fs::write(template_dir.join("load"), consts::T_LOAD).unwrap();

        match process::Command::new("chmod")
            .arg("+x")
            .arg(&template_dir.join("load"))
            .output()
        {
            Ok(_) => {}
            Err(_) => return Err("Failed to set load script as executable".to_string()),
        };

        Ok(template_dir)
    }
}
