use clap::Parser;
use std::path::{PathBuf, self};
use expanduser;

#[derive(Parser)]
#[command(version,name="hyprtheme")]
pub enum Hyprtheme{
    Apply(Apply),
}

#[derive(Parser)]
pub struct  Apply{

    #[arg(value_parser=parse_theme)]
    pub theme: PathBuf,

}  

fn parse_theme(theme: &str) -> Result<PathBuf, String> {
    // expand user
    let theme_dir = expanduser::expanduser("~/.config/hypr/themes").unwrap();

    for entry in path::Path::new(&theme_dir).read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if dir_name == theme {
                return Ok(path);
            }
        }
    }
    Err(format!("Theme {} not found", theme))
}