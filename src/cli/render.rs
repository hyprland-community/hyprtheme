use std::path::Path;

use crate::helper::util::ThemeMap;
use owo_colors::OwoColorize;

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
        Ok(_) => println!(
            "{} applied {}",
            "[  ]:".green().bold(),
            config.theme.name.bold()
        ),
        Err(e) => error(e),
    };
}

pub async fn install(theme: String) {
    println!("installing: {}", theme.yellow().bold());
    match crate::util::Repo::install_theme(theme.as_str()).await {
        Ok(_) => println!("{} installed: {}", "[  ]:".green().bold(), theme.bold()),
        Err(e) => error(e),
    };
}

pub fn warn(msg: &str) {
    println!(
        "{} {}",
        "[ ! ]:".yellow().bold(),
        msg.black().on_yellow().bold()
    );
}

pub fn error(msg: impl Into<String>) {
    println!("{} {}", "[  ]:".red().bold(), msg.into().bold());
}

pub fn template(path: &Path) {
    println!(
        "{} created template at: {}",
        "[  ]:".green().bold(),
        path.display().to_string().blue().bold()
    );
}
