mod cli;
mod consts;
mod theme;
use clap::Parser;
use cli::commands::CliCommands;
use std::process::ExitCode;
use theme::{installed, saved};

#[tokio::main]
async fn main() -> ExitCode {
    match CliCommands::parse() {
        CliCommands::List(arguments) => {
            println!(
                "{}",
                arguments
                    .get_formatted_list()
                    .await
                    .expect("Failed to list the themes")
            );
        }

        CliCommands::Install(arguments) => {
            let installed_theme = arguments.install().await;

            match installed_theme {
                Ok(theme) => {
                    println!(
                        "Installed theme {} to {}\n",
                        &theme.config.meta.name,
                        &arguments.hypr_dir.display()
                    );
                }
                Err(error) => {
                    println!(
                        "Failed to install theme to {}\n{}",
                        &arguments.hypr_dir.display(),
                        error
                    );
                }
            }
        }

        CliCommands::Uninstall(arguments) => {
            let installed_theme = installed::get(Some(&arguments.hypr_dir))
                .await
                .expect("Error retrieving installed theme")
                .expect("No installed theme found");

            let name = &installed_theme.config.meta.name.clone();

            installed_theme
                .uninstall(Some(&arguments.hypr_dir))
                .await
                .expect("Failed to uninstall theme");

            println!("Succesfully uninstalled theme: {}", &name);
        }

        CliCommands::Update(arguments) => {
            let installed_theme = installed::get(Some(&arguments.hypr_dir))
                .await
                .expect("Error retrieving installed theme")
                .expect("No installed theme found")
                .update(Some(&arguments.data_dir))
                .await
                .expect("Failed to update theme");

            println!(
                "Succesfully updated installed theme: {}",
                &installed_theme.config.meta.name
            );
        }

        // Takes in a theme id,
        // as otherwise we won't be able to tell which Gruvbox Theme should be uninstalled,
        // if there are three with the same name, author but hosted on different platforms
        CliCommands::Remove(arguments) => {
            // TODO
            // So this is another problem
            // Let's say we have 3 themes installed.
            // One from Github, then Gitlab and some self-hosted thingy
            // All are called Gruvbox and are from different users, all called Foo.
            // Now we want to remove Gruvbox.
            // Which one do we remove?
            // The one from Gitlab?
            // The one from Github?
            // The one from self-hosted?
            //
            // We need to have an easy to write ID, so that the user can delete this theme
            // Also we need to list the ID on list, too
            saved::find_saved(&arguments.theme_name, Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.")
                .expect("Could not find specified saved theme.")
                .remove()
                .expect("Failed to remove specified theme.");

            println!("Succesfully removed theme  {}", &arguments.theme_name);
        }

        CliCommands::Clean(arguments) => {
            let themes = saved::get_all(Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.");

            for theme in themes {
                let name = theme.config.meta.name.clone();
                if theme
                    .is_installed(Some(&arguments.hypr_dir))
                    .await
                    .expect(format!("Failed to check installed theme: {}", &name).as_str())
                {
                    continue;
                }
                theme
                    .remove()
                    .expect(format!("Failed to remove theme: {}", &name).as_str());
            }

            println!("Succesfully clean unused themes.");
        }

        CliCommands::UpdateAll(arguments) => {
            let themes = saved::get_all(Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.");

            for theme in themes {
                theme.update().expect("Failed to update theme.");
            }

            println!("Succesfully updated all themes.");
        }
    }

    return ExitCode::SUCCESS;
}
