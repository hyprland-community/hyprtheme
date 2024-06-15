use std::path::PathBuf;
use clap::Parser;


#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub enum Hyprtheme {
    Init,
    Enable(Enable),
    Disable(Disable),
    List(List),
    Install(Install),
    Uninstall(Uninstall),
    Update(Update),
    Uri(Uri),
}

#[derive(Parser)]
pub struct Uri {
    pub uri: String,
}

#[derive(Parser)]
pub struct Enable {
    pub theme: String,

    #[arg(short,long,default_value="~/.config/hypr/themes/hyprtheme.conf")]
    pub config: PathBuf,
}

#[derive(Parser)]
pub struct Disable {
    pub theme: String,

    #[arg(short,long,default_value="~/.config/hypr/themes/hyprtheme.conf")]
    pub config: PathBuf,
}

#[derive(Parser)]
pub struct List {

    #[arg(short,long,default_value = "false")]
    pub installed: bool,

    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub theme_dir: PathBuf,
}

#[derive(Parser)]
pub struct Install {
    pub theme: String,

    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub theme_dir: PathBuf,
}

#[derive(Parser)]
pub struct Uninstall {
    pub theme: String,

    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub theme_dir: PathBuf,
}

#[derive(Parser)]
pub struct Update {
    pub theme: String,

    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub theme_dir: PathBuf,
}

fn parse_path(path: &str) -> Result<PathBuf, String> {
    // expand ~
    let path = shellexpand::tilde(path);
    let path = path.as_ref();
    let path = PathBuf::from(path);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("Path does not exist: {}", path.display()))
    }
}
