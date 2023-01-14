use std::fs;

use crate::{helper::consts, parser::config::Config};

pub fn ensure_config() {
    let hypr_config = expanduser::expanduser("~/.config/hypr/hyprland.conf").unwrap();

    let conf = match hypr_config.exists() {
        true => fs::read_to_string(&hypr_config).expect("Unable to read hyprland config file"),
        false => String::new(),
    };

    if !conf.contains(consts::A_CONF) {
        println!("Adding autogen to {}", hypr_config.display());
        fs::write(hypr_config, conf + consts::A_CONF).expect("Unable to write to file");
    }
}

pub fn hyprconf(config: Config) {
    ensure_config();

    let dist_path = expanduser::expanduser("~/.config/hypr/themes/dist.conf").unwrap();

    fs::write(dist_path, config.build_conf()).expect("Unable to write to dist file");
}
