use std::path::Path;

use colored::*;

use crate::helper::util::ThemeMap;

pub fn list_themes_deep(themes: Vec<ThemeMap>) {
    for theme in themes {
        println!(
            "{}   {}",
            theme.name.green().bold(),
            theme.git.unwrap_or_default().blue().underline()
        );
        for subtheme in theme.subthemes {
            println!("• {}", subtheme.name.yellow());
        }
    }
}

pub fn list_themes(themes: Vec<String>) {
    for theme in themes {
        println!("{}", theme.green().bold());
    }
}

pub fn apply(config: crate::parser::config::Config) {
    println!("applying {}", config.theme.name.green().bold());
    match crate::helper::apply::hyprconf(config.to_owned()) {
        Ok(_) => println!("applied {}", &config.theme.name.green().bold()),
        Err(e) => error(e.as_str()),
    };
}

pub async fn install(theme: String) {
    println!("installing: {}", theme.yellow().bold());
    match crate::util::Repo::install_theme(theme.as_str()).await {
        Ok(_) => println!("installed: {}", theme.green().bold()),
        Err(e) => error(e.as_str()),
    };
}

pub fn info(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn warn(msg: &str) {
    println!("{}", msg.black().on_yellow().bold());
}

pub fn error(msg: &str) {
    println!("{}", msg.black().on_red().bold());
}

pub fn template(path: &Path) {
    println!(
        "created template at {}",
        path.display().to_string().blue().bold()
    );
}
