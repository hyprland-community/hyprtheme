use std::fs;

use crate::{helper::consts, parser::config::Config};

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

    let dist_path = expanduser::expanduser("~/.config/hypr/themes/dist.conf").unwrap();

    match fs::write(dist_path, config.build_conf()) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }

    Ok(true)
}
