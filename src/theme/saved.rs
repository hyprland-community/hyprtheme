use super::create_theme_id;
use super::helper::{
    create_hyrptheme_source_string, create_source_string, is_theme_installed, prepend_to_file,
    ThemeId,
};
use super::toml_config::ParsedThemeConfig;
use super::{installed, installed::InstalledTheme};
use crate::consts::{DEFAULT_DOWNLOAD_PATH, DEFAULT_HYPR_CONFIG_PATH};
use anyhow::{anyhow, Context, Ok, Result};
use expanduser::expanduser;
use globset::{Glob, GlobSetBuilder};
use pathdiff::diff_paths;
use serde::Deserialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

/// A theme in the data directory, thus saved to the disc.
/// Can be installed, updated, etc
///
/// That is _*NOT*_ an already installed theme. For that see the InstalledTheme struct
#[derive(Deserialize, Debug)]
pub struct SavedTheme {
    /// The path to the theme in the data directory
    path: PathBuf,

    /// The path to the theme config toml file
    config_path: PathBuf,

    /// Parsed theme config
    pub config: ParsedThemeConfig,
}

impl SavedTheme {
    /// Has an optional `data_dir` argument, which by default is `~/.local/share/hyprtheme/themes`
    pub async fn install(&self, hypr_dir: Option<&PathBuf>) -> Result<InstalledTheme> {
        let hypr_dir = hypr_dir
            .unwrap_or(&expanduser(&DEFAULT_HYPR_CONFIG_PATH).context(format!(
                "Failed to expand home directory for the default install path: {}",
                DEFAULT_HYPR_CONFIG_PATH
            ))?)
            .to_path_buf();

        // It might be harder than expected to save themes in ~/.config/hypr/hyprtheme
        // as the theme .conf files need to get sourced from somewhere,
        // but putting themes in another dir like ~/.config/hyprtheme/themes should work nicely
        let install_dir = &hypr_dir.join("hyprtheme/");
        let _ = fs::create_dir_all(install_dir)
            .context("Failed to create hyprtheme subdirectory in hypr config direcotry.")?;

        let _ = &self.setup_dots(&hypr_dir, install_dir).await?;
        let _ = setup_theme_variables(&hypr_dir)?;
        let _ = &self.run_setup_script(install_dir, &hypr_dir).await?;

        // Copy over the hyprtheme.toml, which gets used to determine which theme is currently installed
        fs::copy(&self.config_path, &install_dir.join("hyprtheme.toml"))?;

        let installed_theme = installed::get(Some(&hypr_dir))
            .await
            .context("Failed to retrieve installed theme directly after installtion")?;

        return match installed_theme {
            Some(theme) => Ok(theme),
            None => Err(anyhow!(format!(
                "Failed to install theme. hyprtheme.conf could not be located in {}. Please open up an issue and we shall fix it!",
                hypr_dir.display()
            ))),
        };
    }

    pub fn get_id(&self) -> ThemeId {
        create_theme_id(&self.config.meta.repo, self.config.meta.branch.as_deref())
    }

    async fn setup_dots(&self, hypr_dir: &PathBuf, install_dir: &PathBuf) -> Result<()> {
        let _ = self.copy_general_dots()?;
        let _ = self.setup_hyprtheme_hypr_dots(&hypr_dir, &install_dir)?;

        // TODO  Prompt and install extra configs

        Ok(())
    }

    async fn run_setup_script(&self, install_dir: &PathBuf, hypr_dir: &PathBuf) -> Result<()> {
        let setup_path_relative = &self.config.lifetime.setup;
        let setup_script_path = &self.path.join(setup_path_relative);

        if !setup_script_path.try_exists()? {
            print!("No setup script found. Skipping...");
            return Ok(());
        }

        std::process::Command::new("bash")
            .env("THEME_DIR", &self.path)
            .env("HYPR_INSTALL_DIR", &install_dir)
            .env("HYPR_CONFIG_DIR", &hypr_dir)
            .arg(&setup_script_path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(&self.path)
            .output()?;

        Ok(())
    }

    /// Move the hypr config dir into `~/.config/hypr/hyprtheme`
    /// And then source it
    fn setup_hyprtheme_hypr_dots(&self, hypr_dir: &PathBuf, install_dir: &PathBuf) -> Result<()> {
        let from_dir = &self.path.join(&self.config.hypr.location);
        let copy_options = fs_extra::dir::CopyOptions {
            overwrite: true,
            skip_exist: false,
            ..Default::default()
        };

        fs_extra::dir::copy(&from_dir, &install_dir, &copy_options).context(format!(
            "Failed to copy over hypr config directory from {} to {}",
            from_dir.display(),
            install_dir.display()
        ))?;

        // Calculate theme source string, like `source=~/.config/hypr/settings.conf`
        // TODO: replace absolute source path with ~ if possible
        // makes config more portable

        let hyprland_config_path = &hypr_dir.join("hyprland.conf");
        let hyprtheme_source_str =
            create_hyrptheme_source_string(&install_dir.join("hyprtheme.conf"));
        // Read out hyprland.conf via std::fs::read_to_string("list.txt").unwrap();
        // Check if hyprtheme.conf is sourced in hyprland.conf, if not, source it
        let is_already_sourced = std::fs::read_to_string(&hyprland_config_path)
            .context(format!(
                "Failed to read out main hyprtheme.conf during hyprland dots setup at {}",
                hyprland_config_path.display()
            ))?
            .contains(&hyprtheme_source_str);

        if !is_already_sourced {
            // Append the source line
            // TODO should be sourced after variables sourcing
            let mut config_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&hyprland_config_path)?;

            config_file.write_all(&hyprtheme_source_str.as_bytes())?;
        }

        Ok(())
    }

    /// Copy over the dot files, as specified in the theme toml.
    ///
    /// Deletes directories/files which would get overwritten to prevent clutter
    ///
    /// Not the hypr dots, as they need special treatment
    fn copy_general_dots(&self) -> Result<()> {
        if self.config.dots.is_empty() {
            // Featured themes always have to follow proper practices, so let's recommend those in this case
            print!(
                "No dots to move specified. This theme probably depends on it's own scripts for this logic. It won't backup your dots, and a clean uninstall is likely not possible, too. We recommend using themes from https://hyprland-community/hyprtheme/browse"
            )
            // TODO prompt to cancel installation.
        }

        for dots_config in &self.config.dots {
            let destination_dir = expanduser(dots_config.to.clone().unwrap_or_else(|| {
                let mut to_path = dots_config.from.clone().display().to_string();
                to_path.insert_str(0, "~/");
                to_path
            }))
            .with_context(|| {
                format!(
                    "Failed to get install path for dot files in: {}",
                    &dots_config.from.display()
                )
            })?;

            let root_dots_path = &self.path.join(&dots_config.from);

            let mut ignore_glob = GlobSetBuilder::new();
            for ignore_pattern in &dots_config.ignore {
                ignore_glob.add(Glob::new(&ignore_pattern).with_context(|| {
                    format!("Invalid ignore glob pattern: {}", &ignore_pattern)
                })?);
            }
            let ignore_glob = ignore_glob.build()?;

            let mut include_glob = GlobSetBuilder::new();
            for include_pattern in &dots_config.include {
                include_glob.add(Glob::new(&include_pattern).with_context(|| {
                    format!("Invalid include glob pattern: {}", &include_pattern)
                })?);
            }
            let include_glob = include_glob.build()?;

            let copy_options_dir = fs_extra::dir::CopyOptions {
                copy_inside: true,
                overwrite: true,
                ..Default::default()
            };

            let copy_options_file = fs_extra::file::CopyOptions {
                skip_exist: false,
                overwrite: true,
                ..Default::default()
            };

            // Get top level folders of to_copy to determine what gets deleted/backed up
            let mut top_level_paths: Vec<(PathBuf, PathBuf)> = Vec::new();
            // Filtered by the globs
            let mut from_to_paths: Vec<(PathBuf, PathBuf)> = Vec::new();
            for (index, entry) in WalkDir::new(&root_dots_path)
                .min_depth(1)
                .into_iter()
                .enumerate()
            {
                let from_path = entry
                    .context(format!(
                        "Invalid file/dir entry in {}",
                        &root_dots_path.display()
                    ))?
                    .path()
                    .to_path_buf();
                let from_path_relative = diff_paths(&from_path, &self.path).expect(
                    format!(
                        "Failed to find relative path for dot file:\n{}\n{}",
                        &from_path.display(),
                        &self.path.display()
                    )
                    .as_str(),
                );

                // Ignore `hypr_directory` files, as they get treated differently
                if from_path_relative.starts_with(&self.config.hypr.location) {
                    continue;
                }

                // File/dir is ignored and not included again, let's ignore it
                if ignore_glob.is_match(&from_path_relative)
                    && !include_glob.is_match(&from_path_relative)
                {
                    continue;
                }

                let to_path = destination_dir.join(from_path.strip_prefix(&root_dots_path)?);

                // The first iteration contains the top level dirs/files
                if index == 0 {
                    top_level_paths.push((from_path.clone(), to_path.clone()));
                }

                from_to_paths.push((from_path, to_path));
            }

            // TODO: Backup existing files
            //      - Check if hyprtheme-theme is installed. If yes, skip,
            //        as we would overwrite the backup with a theme
            //      - If no, save theme into <data>/backup/<date>/

            for (from, to) in top_level_paths {
                // Delete the file/dir which would get overwritten
                // Do nothing if it does not exist (it's not a file nor dir)
                if to.is_file() {
                    fs::remove_file(&to)
                        .context(format!("Failed to remove file: {}", &to.display()))?;
                } else if to.is_dir() {
                    fs::remove_dir_all(&to)
                        .context(format!("Failed to remove directory: {}", &to.display()))?;
                }

                // Copy over the dots
                if from.is_file() {
                    fs_extra::file::copy(from.as_path(), &to.as_path(), &copy_options_file)
                        .context(format!(
                            "Failed to copy file {} to {}",
                            &from.display(),
                            &to.display()
                        ))?;
                } else {
                    fs_extra::dir::copy(from.as_path(), &to.as_path(), &copy_options_dir).context(
                        format!(
                            "Failed to copy direcotry contents of {} to {}",
                            &from.display(),
                            &to.display()
                        ),
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Remove the theme from the data directory, therefore consumes self
    ///
    /// Used to clean up unused themes
    pub fn remove(self) -> Result<()> {
        println!(
            "Uninstalling theme {} from {}",
            &self.config.meta.name,
            &self
                .path
                .to_str()
                .expect("Theme path contained non-unicode characters. Should not happen.")
        );

        Ok(std::fs::remove_dir_all(&self.path)?)
    }

    /// Update the repository in the data directory
    ///
    /// Returns itself in case you want to chain methods
    pub fn update(&self) -> Result<&Self> {
        println!(
            "Updating theme {} in {}",
            &self.config.meta.name,
            &self.path.display()
        );

        // delete dir
        std::process::Command::new("sh")
            .arg("-c")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(&self.path)
            .arg("git pull")
            .output()?;

        return Ok(&self);
    }

    /// Check if this saved theme is also installed
    pub async fn is_installed(&self, config_dir: Option<&PathBuf>) -> Result<bool> {
        is_theme_installed(&self.config.meta.get_id(), config_dir).await
    }
}

// display
impl std::fmt::Display for SavedTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.config.meta.name,)
    }
}

/// Create a theme struct from a hyprtheme repo.
///
/// Like this a themes can be loaded into memory, queried
pub async fn from_directory(path: &PathBuf) -> Result<SavedTheme> {
    let config_path = get_theme_toml_path(path)?;
    let config: ParsedThemeConfig = toml::from_str(fs::read_to_string(&config_path)?.as_str())?;

    // Lets prevent weirdly named themes, like "ФΞд฿Ŀ"
    if !config
        .meta
        .name
        .bytes()
        .all(|character| matches!(character, b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' | b' '))
    {
        return Err(anyhow!("Theme name contains invalid characters. Only latin letters, numbers and '_', '-' are allowed."));
    }

    return Ok(SavedTheme {
        path: path.to_owned(),
        config_path,
        config,
    });
}

/// Get the saved theme in the data directory by its name
///
/// - name: The name of the theme as in its theme.toml config file
/// - data_dir: Data directory of hyprtheme
pub async fn find_saved(
    theme_id: &ThemeId,
    data_dir: Option<&PathBuf>,
) -> Result<Option<SavedTheme>> {
    let theme = get_all(data_dir)
        .await
        .context("Failed to retrieve saved themes (find_saved).")?
        .into_iter()
        .find(|theme| theme.config.meta.get_id() == *theme_id);

    return Ok(theme);
}

/// Gets all themes saved in the data directory
pub async fn get_all(data_dir: Option<&PathBuf>) -> Result<Vec<SavedTheme>> {
    let data_dir: PathBuf = data_dir
        .unwrap_or(&expanduser(DEFAULT_DOWNLOAD_PATH)?)
        .to_owned();

    let themes_dir = data_dir.join("/themes/");
    let entries = fs::read_dir(themes_dir)?;

    let mut themes: Vec<SavedTheme> = vec![];
    for entry in entries {
        themes.push(from_directory(&entry?.path()).await?);
    }

    return Ok(themes);
}

/// Get the toml path from the theme directory.
///
/// Nessecary as there are multiple possible locations for it
fn get_theme_toml_path(theme_dir: &PathBuf) -> Result<PathBuf> {
    let locations = [".hyprtheme/hyprtheme.toml", "hyprtheme.toml"];

    for location in &locations {
        let path = theme_dir.join(location);
        println!("Checking for hyprtheme.toml in {}", path.display());
        match path.try_exists() {
            Result::Ok(true) => return Ok(path),
            Result::Ok(false) => continue, // File does not exist, try the next location
            Err(e) => return Err(e.into()), // Propagate any errors that occurred during the check
        }
    }

    Err(anyhow!(
        "hyprtheme.toml not found in any of the default locations"
    ))
}

/// Add variables.conf and source it in the hypr directory
fn setup_theme_variables(hypr_dir: &PathBuf) -> Result<()> {
    let hyprland_config_path = &hypr_dir.join("hyprland.conf");

    let filename = "variables.conf";
    let file_path = hypr_dir.join(filename);

    if !file_path.try_exists()? {
        let default_variables = { include_str!("default_variables.conf") };

        fs::write(&file_path, default_variables)?;
    }

    let variables_source_str = create_source_string(&file_path, hypr_dir);

    let is_already_sourced = std::fs::read_to_string(&hyprland_config_path)
        .context("Failed to read out main hyprtheme.conf during variables setup")?
        .contains(&variables_source_str);

    if !is_already_sourced {
        // Append the source line
        prepend_to_file(&file_path, &variables_source_str)?;
    }

    Ok(())
}
