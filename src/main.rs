use clap::Parser;

mod cli;
mod util;

use util::repo;
use util::ansi::{red, reset,bold};
use util::config::{Module,Config};

use cli::parse::Hyprtheme;

use expanduser::expanduser;

use std::fmt::format;
use std::{path::PathBuf, process::ExitCode};


async fn install_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
    let theme = match repo::find_theme(&theme,&theme_dir).await {
        Ok(theme) => theme,
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    };
    println!("found {}", theme);

    match theme.install(Some(theme_dir)) {
        Ok(_) => println!("\ninstalled"),
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    }
    ExitCode::SUCCESS
}

async fn uninstall_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
    let theme = match repo::find_theme(&theme,&theme_dir).await {
        Ok(theme) => theme,
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    };
    println!("found {}", theme);

    match theme.uninstall(Some(theme_dir)) {
        Ok(_) => println!("\nuninstalled"),
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    }
    ExitCode::SUCCESS
}

async fn update_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
    let theme = match repo::find_theme(&theme,&theme_dir).await {
        Ok(theme) => theme,
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    };
    println!("found {}", theme);

    match theme.update(Some(theme_dir)) {
        Ok(_) => println!("\nupdated"),
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    }
    ExitCode::SUCCESS
}


#[tokio::main]
async fn main() -> ExitCode{
    match Hyprtheme::parse() {
        Hyprtheme::Init => {
            let mut config = Config::new();
            config.path = expanduser("~/.config/hypr/themes/hyprtheme.conf").unwrap();
            config.ensure_exists().unwrap();

            let hyprland_conf = expanduser("~/.config/hypr/hyprland.conf").unwrap();

            let content = match std::fs::read_to_string(&hyprland_conf) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            };

            let source_line = format!("source={}", config.path.display());

            for line in content.lines() {
                if line.trim() == source_line.trim() {
                    println!("source line already exists");
                    return ExitCode::SUCCESS;
                }
            }

            println!("adding source line");
            std::fs::write(&hyprland_conf, format!("{}\n\n{}",source_line,content)).unwrap();
        },
        Hyprtheme::Enable(enable) => {
            let mut config = Config::from(expanduser(enable.config.to_str().unwrap()).unwrap().to_owned()).await;
            match config.add_module(Module::new(None, enable.config.parent().unwrap().join(enable.theme))) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            };
            match config.apply() {
                Ok(_) => println!("enabled"),
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            }
        },
        Hyprtheme::Disable(disable) => {
            let mut config = Config::from(expanduser(disable.config.to_str().unwrap()).unwrap().to_owned()).await;
            match config.cleanup() {
                Ok(_) => println!("cleanup done"),
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            }
            match config.remove_module(Module::new(None, disable.config.parent().unwrap().join(disable.theme))) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            };
            match config.apply() {
                Ok(_) => println!("disabled"),
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            }
        },
        Hyprtheme::List(list) => {
            for theme in repo::fetch_themes(&list.theme_dir,None).await.unwrap().themes {
                println!("{}", theme);
            }
        },
        Hyprtheme::Install(install) => {
            return install_theme(install.theme, install.theme_dir).await
        },
        Hyprtheme::Uninstall(uninstall) => {
           return uninstall_theme(uninstall.theme, uninstall.theme_dir).await
        },
        Hyprtheme::Update(update) => {
            return update_theme(update.theme, update.theme_dir).await
        },
        Hyprtheme::Uri(uri) => {
            match uri.uri.strip_prefix("hyprtheme://") {
                Some(uri) => {
                    let uri = uri.split('+').collect::<Vec<&str>>();

                    if uri.len() < 2 {
                        eprintln!("{}{}{}",reset() + &red(false) + &bold() ,"Invalid uri",reset());
                        return ExitCode::FAILURE;
                    }
                    
                    let command = uri[0];
                    let theme = uri[1];
                    
                    let theme_dir = if uri.len() > 2 {
                        PathBuf::from(uri[2])
                    } else {
                        expanduser("~/.config/hypr/themes").unwrap()
                    };

                    match command.to_lowercase().as_str() {
                        "install" => {
                            install_theme(String::from(theme), theme_dir).await;
                        },
                        "uninstall" => {
                            uninstall_theme(String::from(theme), theme_dir).await;
                        },
                        _ => {
                            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,"Invalid command",reset());
                            return ExitCode::FAILURE;
                        },
                    }
                },
                None => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,"Invalid uri",reset());
                    return ExitCode::FAILURE;
                },
            }
        },
    }
    return ExitCode::SUCCESS;
}
