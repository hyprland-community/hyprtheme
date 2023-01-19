use std::{fs::{self}, process::Command};

use crate::{helper::consts, parser::config::Config, util::Util};

pub fn ensure_config() -> Result<bool, String> {
    let hypr_config = expanduser::expanduser("~/.config/hypr/hyprland.conf").unwrap();

    let conf = match hypr_config.exists() {
        true => match fs::read_to_string(&hypr_config) {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        },
        false => String::new(),
    };

    if !conf.contains(consts::A_CONF) {
        println!("Adding autogen to {}", hypr_config.display());
        match fs::write(hypr_config, conf + consts::A_CONF) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        };
    }

    Ok(true)
}

pub fn hyprconf(config: Config) -> Result<bool, String> {
    match ensure_config() {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    let dist_path = expanduser::expanduser("~/.config/hypr/themes/dist/").unwrap();

    match fs::create_dir_all(&dist_path){
        Ok(_) => {}
        Err(e) => return Err(format!("error creating all dirs: {}",e.to_string())),
    };

    Util::kill_all_bars(Some(config.theme._kill.exclude_bar.to_owned()));
    Util::kill_all_wallpapers(Some(config.theme._kill.exclude_wallpaper.to_owned()));

    if dist_path.join("cleanup.sh").is_file() {
        match Command::new(dist_path.join("cleanup.sh")).arg(&config.theme._path).spawn() {
            Ok(mut cmd) => match cmd.wait(){
                Ok(_) => {}
                Err(e) => return Err(format!("error while running cleanup.sh : {}",e.to_string())),
            },
            Err(e) => return Err(format!("error running cleanup.sh : {}",e.to_string())),
        };
    }

    match fs::write(dist_path.join("dist.conf"), &config.build_conf()) {
        Ok(_) => {}
        Err(e) => return Err(format!("error writing dist.conf: {}",e.to_string())),
    }

    if config.theme._script.cleanup.is_file(){
        match fs::copy(config.theme._script.cleanup, dist_path.join("cleanup.sh")){
            Ok(_) => {}
            Err(e) => return Err(format!("error copying cleanup.sh: {}",e.to_string())),
        };
        match Command::new("chmod")
        .arg("+x")
        .arg(dist_path.join("cleanup.sh"))
        .spawn() {
            Ok(_) => {}
            Err(e) => return Err(format!("error on setting cleanup.sh as executable: {}",e.to_string())),
        }
    } else if dist_path.join("cleanup.sh").is_file(){
        match fs::remove_file(dist_path.join("cleanup.sh")){
                Ok(_) => {}
                Err(e) => return Err(format!("error removing cleanup.sh: {}",e.to_string())),
        };
    } 
    

    

    Ok(true)
}
