mod cli_flags;
mod consts;
mod theme;
use anyhow::{Context, Result};
use clap::Parser;
use cli_flags::flags::CliFlags;
use expanduser::expanduser;
use std::{fs, process::ExitCode};
use theme::{
    installed,
    saved::{self},
};

#[tokio::main]
async fn main() -> ExitCode {
    ensure_default_dirs_exist().expect("Failed to create default directories");

    match CliFlags::parse() {
        CliFlags::List(arguments) => {
            println!(
                "{}",
                arguments
                    .get_formatted_list()
                    .await
                    .expect("Failed to list the themes")
            );
        }

        CliFlags::Install(arguments) => {
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

        CliFlags::Uninstall(arguments) => {
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

        CliFlags::Update(arguments) => {
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

        CliFlags::Remove(arguments) => arguments.remove().await.expect("Failed to remove theme"),

        CliFlags::Clean(arguments) => {
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

        CliFlags::UpdateAll(arguments) => {
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
