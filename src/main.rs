/// This file is named `theme_config.rs` as we might want to add
/// a config file for Hyprtheme itself in the future
mod cli;
use crate::cli::commands::ThemeInstallName;
mod consts;
mod theme;
use clap::Parser;
use cli::commands::CliCommands;
use expanduser::expanduser;
use std::{path::PathBuf, process::ExitCode};
use theme::installed::InstalledTheme;
use theme::online;
use theme::saved::SavedTheme;
use theme::{installed, saved};

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
            // Get the clone string
            let git_url: String = match arguments.name {
                ThemeInstallName::Featured(theme) => online::get_theme(&theme, &arguments.data_dir)
                    .await
                    .expect("Could not find provided theme."),
                ThemeInstallName::Git((author, repo)) => {
                    "git@github.com:".to_string() + &author + "/" + &repo + ".git"
                }
                ThemeInstallName::Github(github_string) => github_string,
            };

            // TODO what to do if the theme is already installed?
            // It is kinda hard to figure out the name of the theme by the arguments,
            // as the repo might not have the same name, and how would we enforce that
            theme::saved::SavedTheme::download(
                &git_url,
                arguments.branch.as_ref(),
                Some(&arguments.data_dir),
            );

            // TODO: I kinda missed the download->install thing in theme::install uhmm..
            // If already saved, install it
            // Otherwise download and then install it
        }

        CliCommands::Uninstall(arguments) => {
            installed::get(Some(&arguments.config_dir))
                .expect("Error retrieving installed theme")
                .expect("No installed theme found")
                .uninstall()
                .expect("Failed to uninstall theme");
        }

        CliCommands::Update(arguments) => {
            installed::get(Some(&arguments.theme_dir))
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
