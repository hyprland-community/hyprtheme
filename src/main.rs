mod cli;
mod consts;
mod theme;
mod helper;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{CliParser,CliCommands};
use expanduser::expanduser;
use std::{fs, process::ExitCode};

use theme::{fetch_all, fetch_all_installed, fetch_online, ThemeType};
use helper::{identify_offline_theme,identify_theme};

async fn parse_cli() -> Result<()> {
    let cli = CliParser::parse();

    let hypr_dir = cli.hypr_dir;
    let theme_dirs = cli.theme_dirs;
    let theme_urls = cli.theme_urls;

    // println!("Hypr Dir: {:?}", hypr_dir);
    // println!("Theme Dirs: {:?}", theme_dirs);
    // println!("Theme URLs: {:?}\n\n", theme_urls);

    match cli.commands {
        CliCommands::List(mut flags) => {
            println!("listing themes");

            if !flags.installed && !flags.online {
                flags.installed = true;
                flags.online = true;
            }
            
            let mut blacklist_ids = Vec::new();

            match fetch_all_installed(&theme_dirs).await {
                Ok(themes) => {
                    for theme in themes {
                        if flags.installed && theme.get_type_string() == "installed" {
                            println!("{}",theme);
                            blacklist_ids.push(theme.get_id());
                        }
                    }
                },
                Err(e) => {
                    println!("Error fetching installed themes: {}", e);
                }
            
            }

            if flags.online {

                if flags.show_installed{
                    blacklist_ids.clear();
                }

                match fetch_online(theme_urls, Some(blacklist_ids)).await {
                    Ok(themes) => {
                        for theme in themes {
                            let theme: Box<dyn ThemeType> = Box::new(theme);
                            println!("{}",theme);
                        }
                    },
                    Err(e) => {
                        println!("Error fetching online themes: {}", e);
                    }
                }
            }
        }
        CliCommands::Install(flags) => {

            match flags.theme_id {
                Some(theme_id) => {

                    match identify_theme(&theme_id, &theme_dirs, &theme_urls).await {
                        Ok(theme) => {

                            match theme.as_any() {
                                t if t.is::<theme::online::OnlineTheme>() => {
                                    let theme = t.downcast_ref::<theme::online::OnlineTheme>().unwrap().to_owned();
                                    println!("Installing theme: {}", theme.get_name());
                                    match theme.download(&theme_dirs[0]) {
                                        Ok(installed) => {
                                            println!("Theme installed at {}",installed.path.display());
                                        },
                                        Err(e) => {
                                            println!("Error installing theme: {}", e);
                                        }
                                    }
                                },
                                _ => {
                                    println!("Theme already installed");
                                }
                            }

                        },
                        Err(e) => {
                            println!("Couldnt identify theme: {}", e);
                        }
                    }
                },
                None => {
                    todo!("installing themes from git repo directly")
                }
            }
        }
        CliCommands::Uninstall(flags) => {

            match identify_offline_theme(&flags.theme_id, &theme_dirs).await {
                Ok(theme) => {
                    println!("Uninstalling theme: {}", theme.get_name());
                    match theme.as_any() {
                        t if t.is::<theme::installed::InstalledTheme>() => {
                            let theme = t.downcast_ref::<theme::installed::InstalledTheme>().unwrap().to_owned();
                            match theme.uninstall() {
                                Ok(_) => {
                                    println!("Theme uninstalled");
                                },
                                Err(e) => {
                                    println!("Error uninstalling theme: {}", e);
                                }
                            }
                        },
                        _ => {
                            println!("Theme cant be uninstalled");
                        }

                    }
                },
                Err(e) => {
                    println!("Error uninstalling theme: {}", e);
                }
            }

        }
        CliCommands::Update(flags) => {
            match identify_offline_theme(&flags.theme_id, &theme_dirs).await {
                Ok(theme) => {
                    println!("Updating theme: {}", theme.get_name());
                    match theme.as_any() {

                        t if t.is::<theme::installed::InstalledTheme>() => {
                            let theme = t.downcast_ref::<theme::installed::InstalledTheme>().unwrap().to_owned();
                            match theme.update() {
                                Ok(_) => {
                                    println!("Theme updated");
                                },
                                Err(e) => {
                                    println!("Error updating theme: {}", e);
                                }
                            }
                        },
                        _ => {
                            println!("Theme cant be updated");
                        }

                    }
                },
                Err(e) => {
                    println!("Error updating theme: {}", e);
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    ensure_default_dirs_exist().expect("Failed to create default directories");

    parse_cli().await.expect("Failed to parse CLI arguments");

    return ExitCode::SUCCESS;
}

fn ensure_default_dirs_exist() -> Result<()> {
    let _ = fs::create_dir_all(
        expanduser(consts::DEFAULT_DOWNLOAD_PATH)
            .context("Failed to expand default download path.")?,
    )?;
    let _ = fs::create_dir_all(
        expanduser(consts::DEFAULT_HYPR_CONFIG_PATH)
            .context("Failed to expand default hypr config path.")?,
    )?;

    Ok(())
}
