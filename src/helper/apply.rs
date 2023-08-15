use owo_colors::OwoColorize;
use std::{fs, process::Command};

use crate::{helper::consts, parser::config::Config, util::Util};

pub fn ensure_config() -> Result<bool, String> {
    let hypr_config =
        expanduser::expanduser(format!("{}/.config/hypr/hyprland.conf", env!("HOME"))).unwrap();

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

    let dist_path =
        expanduser::expanduser(format!("{}/.config/hypr/themes/dist/", env!("HOME"))).unwrap();

    match fs::create_dir_all(&dist_path) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "{} error creating all dirs: {}",
                "[  ]:".red().bold(),
                e
            ))
        }
    };

    Util::kill_all_bars(Some(config.theme._kill.exclude_bar.to_owned()));
    Util::kill_all_wallpapers(Some(config.theme._kill.exclude_wallpaper.to_owned()));

    let script_conf = config.theme._script.clone();
    if script_conf.load.is_file() {
        match Command::new("chmod")
            .arg("+x")
            .arg(&script_conf.load)
            .spawn()
        {
            Ok(mut cmd) => match cmd.wait() {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!(
                        "{} error while running chmod +x for load script : {}",
                        "[  ]:".red().bold(),
                        e
                    ))
                }
            },
            Err(e) => {
                return Err(format!(
                    "{} error running chmod +x for load script : {}",
                    "[  ]:".red().bold(),
                    e
                ))
            }
        };
        match Command::new(script_conf.load)
            .arg(&config.theme._path)
            .spawn()
        {
            Ok(mut cmd) => match cmd.wait() {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!(
                        "{} error while running load.sh : {}",
                        "[  ]:".red().bold(),
                        e
                    ))
                }
            },
            Err(e) => {
                return Err(format!(
                    "{} error running load.sh : {}",
                    "[  ]:".red().bold(),
                    e
                ))
            }
        };
    }

    if dist_path.join("cleanup.sh").is_file() {
        match Command::new(dist_path.join("cleanup.sh"))
            .arg(&config.theme._path)
            .spawn()
        {
            Ok(mut cmd) => match cmd.wait() {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!(
                        "{} error while running cleanup.sh : {}",
                        "[  ]:".red().bold(),
                        e
                    ))
                }
            },
            Err(e) => {
                return Err(format!(
                    "{} error running cleanup.sh : {}",
                    "[  ]:".red().bold(),
                    e
                ))
            }
        };
    }

    match fs::write(dist_path.join("dist.conf"), config.build_conf()) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "{} error writing dist.conf: {}",
                "[  ]:".red().bold(),
                e
            ))
        }
    }

    if script_conf.cleanup.is_file() {
        match fs::copy(script_conf.cleanup, dist_path.join("cleanup.sh")) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!(
                    "{} error copying cleanup.sh: {}",
                    "[  ]:".red().bold(),
                    e
                ))
            }
        };
        match Command::new("chmod")
            .arg("+x")
            .arg(dist_path.join("cleanup.sh"))
            .spawn()
        {
            Ok(_) => {}
            Err(e) => {
                return Err(format!(
                    "{} error on setting cleanup.sh as executable: {}",
                    "[  ]:".red().bold(),
                    e
                ))
            }
        }
    } else if dist_path.join("cleanup.sh").is_file() {
        match fs::remove_file(dist_path.join("cleanup.sh")) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!(
                    "{} error removing cleanup.sh: {}",
                    "[  ]:".red().bold(),
                    e
                ))
            }
        };
    }

    Ok(true)
}
