/// This file is named `theme_config.rs` as we might want to add
/// a config file for Hyprtheme itself in the future
mod cli;
mod consts;
mod theme;
use clap::Parser;
use cli::commands::CliCommands;
use expanduser::expanduser;
use std::{path::PathBuf, process::ExitCode};
use theme::repo;
use theme::theme::Theme;

#[tokio::main]
async fn main() -> ExitCode {
    match CliCommands::parse() {
        CliCommands::Init => {
            let mut config = Theme::new();
            config.path = expanduser("~/.config/hypr/themes/hyprtheme.conf").unwrap();
            config.ensure_exists().unwrap();

            let hyprland_conf = expanduser("~/.config/hypr/hyprland.conf").unwrap();

            let content = match std::fs::read_to_string(&hyprland_conf) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            };

            let source_line = format!("source={}", config.path.display());

            for line in content.lines() {
                if line.trim() == source_line.trim() {
                    println!("source line already exists");
                    return ExitCode::SUCCESS;
                }
            }

            println!("adding source line");
            std::fs::write(&hyprland_conf, format!("{}\n\n{}", source_line, content)).unwrap();
        }
        CliCommands::Enable(enable) => {
            let mut config = Theme::from(
                expanduser(enable.config.to_str().unwrap())
                    .unwrap()
                    .to_owned(),
            )
            .await;
            match config.add_module(Module::new(
                None,
                enable.config.parent().unwrap().join(enable.theme),
            )) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            };
            match config.apply() {
                Ok(_) => println!("enabled"),
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
        CliCommands::Disable(disable) => {
            let mut config = Theme::from(
                expanduser(disable.config.to_str().unwrap())
                    .unwrap()
                    .to_owned(),
            )
            .await;
            match config.cleanup() {
                Ok(_) => println!("cleanup done"),
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            }
            match config.remove_module(Module::new(
                None,
                disable.config.parent().unwrap().join(disable.theme),
            )) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            };
            match config.apply() {
                Ok(_) => println!("disabled"),
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
        CliCommands::List(list) => {
            for theme in repo::fetch_themes(&list.theme_dir, None)
                .await
                .unwrap()
                .themes
            {
                println!("{}", theme);
            }
        }
        CliCommands::Install(install) => {
            return install_theme(install.theme, install.theme_dir).await
        }
        CliCommands::Uninstall(uninstall) => {
            return uninstall_theme(uninstall.theme, uninstall.theme_dir).await
        }
        CliCommands::Update(update) => return update_theme(update.theme, update.theme_dir).await,
        CliCommands::Uri(uri) => match uri.uri.strip_prefix("hyprtheme://") {
            Some(uri) => {
                let uri = uri.split('+').collect::<Vec<&str>>();

                if uri.len() < 2 {
                    eprintln!("{}", "Invalid uri",);
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
                    }
                    "uninstall" => {
                        uninstall_theme(String::from(theme), theme_dir).await;
                    }
                    _ => {
                        eprintln!("{}", "Invalid command",);
                        return ExitCode::FAILURE;
                    }
                }
            }
            None => {
                eprintln!("{}", "Invalid uri",);
                return ExitCode::FAILURE;
            }
        },
    }
    return ExitCode::SUCCESS;
}

async fn install_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
    let theme = match repo::find_theme(&theme, &theme_dir).await {
        Ok(theme) => theme,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };
    println!("found {}", theme);

    match theme.install(Some(theme_dir)) {
        Ok(_) => println!("\ninstalled"),
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

async fn uninstall_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
    let theme = match repo::find_theme(&theme, &theme_dir).await {
        Ok(theme) => theme,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };
    println!("found {}", theme);

    match theme.uninstall(Some(theme_dir)) {
        Ok(_) => println!("\nuninstalled"),
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

async fn update_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
    let theme = match repo::find_theme(&theme, &theme_dir).await {
        Ok(theme) => theme,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };
    println!("found {}", theme);

    match theme.update() {
        Ok(_) => println!("\nupdated"),
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
