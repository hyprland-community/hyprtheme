mod cli;
mod consts;
// mod theme;
mod modules;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{CliParser,CliCommands};
use expanduser::expanduser;
use std::{fs, process::ExitCode};
use reqwest::Client;
use serde::Deserialize;


// use theme::{
//     installed,
//     saved::{self},
// };

use modules::theme::{fetch_themes};

async fn parse_cli() -> Result<()> {
    let cli = CliParser::parse();

    let hypr_dir = cli.hypr_dir;
    let themes_dir = cli.themes_dir;

    match cli.commands {
        CliCommands::Clean(flags) => {}
        CliCommands::Install(flags) => {}
        CliCommands::List(flags) => {
            println!("listing themes");
            
            match fetch_themes(None,None).await {
                Ok(themes) => {
                    for theme in themes {

                        let theme_type = match theme.as_any() {
                            t if t.is::<modules::theme::legacy::LegacyTheme>() => "Legacy",
                            t if t.is::<modules::theme::installed::InstalledTheme>() => "Installed",
                            t if t.is::<modules::theme::online::OnlineTheme>() => "Online",
                            _ => "Unknown",
                        };

                        println!("Theme: {}::{} [{}]", theme.get_name(),{theme.get_id()},theme_type);
                    }
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        CliCommands::Remove(flags) => {}
        CliCommands::Uninstall(flags) => {}
        CliCommands::Update(flags) => {}
        CliCommands::UpdateAll(flags) => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    ensure_default_dirs_exist().expect("Failed to create default directories");

    parse_cli().await.expect("Failed to parse CLI arguments");

    // match arguments.args {
    //     CliFlags::List(arguments) => {
    //         println!(
    //             "{}",
    //             arguments
    //                 .get_formatted_list()
    //                 .await
    //                 .expect("Failed to list the themes")
    //         );
    //     }

    //     CliFlags::Install(arguments) => {
    //         let installed_theme = arguments.install().await.expect("Failed to install theme.");

    //         println!("Installed theme {}", installed_theme.config.meta.name);
    //     }

    //     CliFlags::Uninstall(arguments) => {
    //         let installed_theme = installed::get(arguments.hypr_dir.as_ref())
    //             .await
    //             .expect("Error retrieving installed theme")
    //             .expect("No installed theme found");

    //         let name = &installed_theme.config.meta.name.clone();

    //         installed_theme
    //             .uninstall(arguments.hypr_dir.as_ref())
    //             .await
    //             .expect("Failed to uninstall theme");

    //         println!("Succesfully uninstalled theme: {}", &name);
    //     }

    //     CliFlags::Update(arguments) => {
    //         let installed_theme = installed::get(arguments.hypr_dir.as_ref())
    //             .await
    //             .expect("Error retrieving installed theme")
    //             .expect("No installed theme found")
    //             .update(arguments.data_dir.as_ref())
    //             .await
    //             .expect("Failed to update theme");

    //         println!(
    //             "Succesfully updated installed theme: {}",
    //             &installed_theme.config.meta.name
    //         );
    //     }

    //     CliFlags::Remove(arguments) => arguments.remove().await.expect("Failed to remove theme"),

    //     CliFlags::Clean(arguments) => {
    //         let themes = saved::get_all(arguments.data_dir.as_ref())
    //             .await
    //             .expect("Failed to lookup saved themes.");

    //         for theme in themes {
    //             let name = theme.config.meta.name.clone();
    //             if theme
    //                 .is_installed(arguments.hypr_dir.as_ref())
    //                 .await
    //                 .expect(format!("Failed to check installed theme: {}", &name).as_str())
    //             {
    //                 continue;
    //             }
    //             theme
    //                 .remove()
    //                 .expect(format!("Failed to remove theme: {}", &name).as_str());
    //         }

    //         println!("Succesfully clean unused themes.");
    //     }

    //     CliFlags::UpdateAll(arguments) => {
    //         let themes = saved::get_all(arguments.data_dir.as_ref())
    //             .await
    //             .expect("Failed to lookup saved themes.");

    //         for theme in themes {
    //             theme.update().expect("Failed to update theme.");
    //         }

    //         println!("Succesfully updated all themes.");
    //     }
    // }

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
