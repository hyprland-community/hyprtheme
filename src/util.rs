use crate::{theme::Theme,consts};
use expanduser;
use std::{fs,path::{self, PathBuf, Path}, io::BufReader,process};
use gitmodules;
use reqwest;

pub struct ThemeMap {
    pub name: String,
    pub subthemes: Vec<ThemeMap>,
}

impl ThemeMap {
    pub fn from_theme(theme: Theme) -> ThemeMap {
        let mut subthemes: Vec<ThemeMap> = Vec::new();

        for subtheme in theme.subthemes.iter() {
            subthemes.push(ThemeMap::from_theme(subtheme.clone()));
        }
        ThemeMap {
            name: theme.name,
            subthemes: subthemes,
        }
    }

    pub fn display(&self) -> String {
        let mut out = self.name.to_owned();
        if self.subthemes.len() > 0 {
            out.push_str(":");

            for subtheme in self.subthemes.iter() {
                let subtheme_display = subtheme.display();
                let l = subtheme_display
                    .split("\n")
                    .into_iter()
                    .collect::<Vec<&str>>();
                for line in l.iter() {
                    out.push_str(&format!("\n  {}", line))
                }
            }
        }
        out
    }
}

pub fn get_subtheme(theme: &Theme) -> Option<Theme> {
    let subtheme_str = &theme.default_subtheme;

    if !subtheme_str.trim().is_empty() {
        let nest = subtheme_str.split(":").into_iter().collect::<Vec<&str>>();

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
        if path.is_dir() {
            if path.join("theme.toml").exists() {
                themes.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
    }
    themes
}

pub fn list_themes_deep() -> Vec<ThemeMap> {
    let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();
    let mut themes: Vec<ThemeMap> = Vec::new();

    for entry in theme_dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let theme_path = path.join("theme.toml");
            if theme_path.exists() {
                themes.push(ThemeMap::from_theme(Theme::from_file(theme_path)));
            }
        }
    }
    themes
}

pub struct Repo {}

impl Repo {
    pub fn list_themes() -> Vec<Theme>{
        let modules_url = "https://raw.githubusercontent.com/hyprland-community/theme-repo/main/.gitmodules";
        let raw = reqwest::blocking::get(modules_url).unwrap().text().unwrap();
        let bytereader = BufReader::new(raw.as_bytes());
        let modules = gitmodules::read_gitmodules(bytereader).unwrap();
        let mut themes = Vec::new();
        for module in modules.iter() {
            let theme_url = module.entries().iter().find(|e| e.0 == "url").unwrap().to_owned().1; 
            let theme_info = reqwest::blocking::get(theme_url.to_owned() + "/blob/master/theme.toml?raw=true").unwrap().text().unwrap();
            let mut theme = Theme::from_string(theme_info,Path::new("/tmp").to_path_buf());
            if theme._repo.is_none() {
                theme._repo = Some(theme_url);
            }
            themes.push(theme);
        }
        themes
    }

    pub fn list_themes_deep() -> Vec<ThemeMap> {
        let mut themes: Vec<ThemeMap> = Vec::new();
        for theme in Repo::list_themes().iter() {
            themes.push(ThemeMap::from_theme(theme.clone()));
        }
        themes
    }

    pub fn install_theme(theme_name:&str) -> Result<PathBuf,String> {
        let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();

        let themes = Repo::list_themes();

        let theme = themes.iter().find(|t| t.name == theme_name);

        match theme {
            Some(t) => {
                process::Command::new("git")
                    .arg("clone")
                    .arg(t._repo.as_ref().unwrap())
                    .arg(&theme_dir.join(&t.name))
                    .output()
                    .expect("failed to clone theme");
                process::Command::new("chmod")
                    .arg("+x")
                    .arg(&theme_dir.join(&t.name).join("load"))
                    .output()
                    .expect("failed to set executable");
                Ok(theme_dir.join(&t.name))
            }
            None => {
                Err(format!("Theme {} not found", theme_name))
            }
        }
    }
}

pub struct Util {}

impl Util {

    pub fn kill_all(){
        Util::kill_all_bars();
        Util::kill_all_wallpapers();
    }

    pub fn kill_all_bars(){
        let pkill = vec!["swaybar","waybar","ironbar"];

        for p in pkill.iter() {
            match process::Command::new("pkill")
                .arg(p)
                .spawn(){
                    Ok(_) => println!("killed {}", p),
                    Err(_) => println!("{} not running", p)
                }
        }

        match process::Command::new("eww")
            .arg("kill")
            .spawn() {
                Ok(_) => println!("killed eww"),
                Err(_) => println!("eww not running")
            }
    }

    pub fn kill_all_wallpapers(){
        let pkill = vec!["swaybg","wbg","hyprpaper","mpvpaper","swww"];

        for p in pkill.iter() {
            match process::Command::new("pkill")
                .arg(p)
                .spawn(){
                    Ok(_) => println!("killed {}", p),
                    Err(_) => println!("{} not running", p)
                }
        }
    }

    pub fn create_template(_template_dir:Option<PathBuf>) -> Result<PathBuf,String> {
        let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();
        
        let template_dir: PathBuf;
        match _template_dir {
            Some(_) => {
                template_dir = _template_dir.unwrap();
            },
            None => {
                template_dir = theme_dir.join("template");
            }
        }
        
        if template_dir.exists() && template_dir.is_dir() {
            return Err("Template already exists".to_string())
        }

        fs::create_dir_all(&template_dir).unwrap();
        
        fs::write(template_dir.join("theme.toml"), consts::template_theme_toml ).unwrap();
        fs::write(template_dir.join("theme.conf"), consts::template_theme_conf ).unwrap();
        fs::write(template_dir.join("load"), consts::template_load ).unwrap();

        process::Command::new("chmod")
            .arg("+x")
            .arg(&template_dir.join("load"))
            .output()
            .expect("failed to set executable");

        Ok(template_dir)

    }
}