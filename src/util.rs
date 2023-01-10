use crate::theme::Theme;
use std::{path::{self, PathBuf}};
use expanduser;

pub struct ThemeMap{
    pub name:String,
    pub subthemes:Vec<ThemeMap>,
}

impl ThemeMap{
    pub fn from_theme(theme:Theme)->ThemeMap{
        let mut subthemes:Vec<ThemeMap> = Vec::new();

        for subtheme in theme.subthemes.iter(){
            subthemes.push(ThemeMap::from_theme(subtheme.clone()));
        }
        ThemeMap{
            name: theme.name,
            subthemes: subthemes
                
        }
    }

    pub fn display(&self)->String{
        let mut out = self.name.to_owned();
        if self.subthemes.len() > 0{
            out.push_str(":");

            for subtheme in self.subthemes.iter(){
                let subtheme_display = subtheme.display();
                let l = subtheme_display.split("\n").into_iter().collect::<Vec<&str>>();
                for line in l.iter(){
                    out.push_str(&format!("\n  {}",line))
                }
            }
        }
        out
    }
}

pub fn get_subtheme(theme:&Theme)->Option<Theme>{
    let subtheme_str= &theme.default_subtheme;

    if !subtheme_str.trim().is_empty(){
        let nest = subtheme_str.split(":").into_iter().collect::<Vec<&str>>();

        let mut subtheme = theme.subthemes.iter().find(|t| t.name == nest[0]).unwrap();

        for n in nest.iter().skip(1){
            subtheme = subtheme.subthemes.iter().find(|t| t.name == *n).unwrap();
        }

        return Some(subtheme.clone())
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

pub fn list_themes() -> Vec<String>{
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

pub fn list_themes_deep() -> Vec<ThemeMap>{
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