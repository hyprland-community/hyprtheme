mod cli;
use crate::cli::commands::ThemeInstallName;
mod consts;
mod theme;
use clap::Parser;
use cli::commands::CliCommands;
use expanduser::expanduser;
use std::process::Termination;
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
            struct GitUrlBranch {
                pub url: String,
                pub branch: Option<String>,
            }

            // TODO prompt for update if theme is already installed

            let git_data: GitUrlBranch = match arguments.name {
                ThemeInstallName::Featured(theme) => {
                    let found_theme = saved::find_saved(&theme)
                        .expect("Failed to lookup saved themes")
                        .map(|theme| GitUrlBranch {
                            // TODO this is not the correct URL, it needs to be a @git url
                            url: theme.config.meta.git,
                            branch: None,
                        })
                        .or(online::find_featured(&theme)
                            .await
                            .expect("Failed to fetch featured theme")
                            .map(|theme| GitUrlBranch {
                                url: theme.repo,
                                branch: theme.branch,
                            }))
                        .expect("Failed to find theme by provided name.");

                    GitUrlBranch {
                        url: found_theme.url,
                        branch: arguments.branch.or(found_theme.branch),
                    }
                }

                ThemeInstallName::Github((author, repo)) => GitUrlBranch {
                    url: "git@github.com:".to_string() + &author + "/" + &repo + ".git",
                    branch: arguments.branch,
                },

                ThemeInstallName::Git(github_string) => GitUrlBranch {
                    url: github_string,
                    branch: arguments.branch,
                },
            };

            // If a theme is already installed, we just install it again
            // Lets keep it simple
            theme::online::download(
                &git_data.url,
                git_data.branch.as_ref(),
                Some(&arguments.data_dir),
            )
            .await
            .expect("Failed to download theme")
            .install(Some(&arguments.data_dir))
            .await
            .expect("Failed to install theme");
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
                .await
                .expect("Failed to update theme");
        }

        CliCommands::Remove(arguments) => {
            // TODO
            // remove downloaded theme from the data directory
            // If the installed theme would get removed prompt
            // that this will also uninstall the current theme.
            // Nessecary as otherwise updating the installed theme will be undefined behavior
            // TODO: Update should reinstall the theme current theme in the data dir if removed

            // TODO config dir argument to installed::get
            // Not sure if this is necessary
            //
            // let installed_theme_option: Option<InstalledTheme> =
            //     installed::get(None).unwrap_or(None);
            // if let Some(installed) = installed_theme_option {
            //     installed
            //         .uninstall()
            //         .map_err(|error| println!("Error uninstalling current theme:\n{}", error));
            // }

            saved::find_saved(&arguments.theme_name)
                .expect("Failed to lookup saved themes.")
                .expect("Could not find specified saved theme.")
                .remove()
                .expect("Failed to remove specified theme.")
        }

        CliCommands::Clean(arguments) => {
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
