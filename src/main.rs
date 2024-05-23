/// This file is named `theme_config.rs` as we might want to add
/// a config file for Hyprtheme itself in the future
mod cli;
mod consts;
mod theme;
use clap::Parser;
use cli::commands::CliCommands;
use expanduser::expanduser;
use std::{path::PathBuf, process::ExitCode};
use theme::installed_theme::InstalledTheme;
use theme::repo;
use theme::theme::Theme;
use theme::{installed_theme, theme};

#[tokio::main]
async fn main() -> ExitCode {
    match CliCommands::parse() {
        CliCommands::List(arguments) => {
            // TODO
            // Fetch all featured and saved themes
            // Display maybe like that:
            // - <Theme name> [Installed]
            // - <Theme name> [Saved]
            // - <Theme name> [Saved]
            // - <Theme name>

            // for theme in repo::fetch_themes(&list.theme_dir, None)
            //     .await
            //     .unwrap()
            //     .themes
            // {
            //     println!("{}", theme);
            // }
        }

        CliCommands::Install(arguments) => {
            // TODO: I kinda missed the download->install thing in theme::install uhmm..
            // If already saved, install it
            // Otherwise download and then install it
        }

        CliCommands::Uninstall(arguments) => {
            installed_theme::get(Some(&arguments.config_dir))
                .expect("Error retrieving installed theme")
                .expect("No installed theme found")
                .uninstall()
                .expect("Failed to uninstall theme");
        }

        CliCommands::Update(arguments) => {
            installed_theme::get(Some(&arguments.theme_dir))
                .expect("Error retrieving installed theme")
                .expect("No installed theme found")
                .update()
                .expect("Failed to update theme");
        }

        CliCommands::Clean(arguments) => {
            // TODO
            // remove downloaded theme from the data directory
        }

        CliCommands::CleanAll(arguments) => {
            // TODO
            // remove all downloaded themes except the installed one from the data directory
        }

        CliCommands::UpdateAll(arguments) => {
            // TODO
            // update all saved themes, including the currently installed one
        }
    }

    return ExitCode::SUCCESS;
}

// async fn install_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
//     let theme = match repo::find_theme(&theme, &theme_dir).await {
//         Ok(theme) => theme,
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     };
//     println!("found {}", theme);

//     match theme.install(Some(theme_dir)) {
//         Ok(_) => println!("\ninstalled"),
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     }
//     ExitCode::SUCCESS
// }

// async fn uninstall_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
//     let theme = match repo::find_theme(&theme, &theme_dir).await {
//         Ok(theme) => theme,
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     };
//     println!("found {}", theme);

//     match theme.uninstall(Some(theme_dir)) {
//         Ok(_) => println!("\nuninstalled"),
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     }
//     ExitCode::SUCCESS
// }

// async fn update_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
//     let theme = match repo::find_theme(&theme, &theme_dir).await {
//         Ok(theme) => theme,
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     };
//     println!("found {}", theme);

//     match theme.update() {
//         Ok(_) => println!("\nupdated"),
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     }
//     ExitCode::SUCCESS
// }
