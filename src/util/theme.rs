use std::fs;
use std::path::PathBuf;

use crate::util::ansi::{bold, green, reset};
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

use expanduser::expanduser;

const DEFAULT_INSTALL_PATH: &str = "~/.config/hypr/themes";
/// The path theme repos are downloaded to
/// TODO use the directories crate to figure out the correct path. The user might have changed it (its Linux after all)
const DEFAULT_DOWNLOAD_PATH: &str = "~/.local/hypr/themes";

#[derive(Deserialize, Debug)]
pub struct Themes {
    pub themes: Vec<Theme>,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    meta: ThemeMeta,
    version: String,
    files: ConfigFilesMeta,
    lifetime: LifeTimeConfig,
    extra_configs: Vec<ExtraConfig>,
    dependencies: Vec<String>,
}
// Maybe I have `Theme` and `ThemeDir`.
// I clone the repo and the installation is just adding the files to ./hypr and sourcing them
// Why do I even need a `install` method here?
// Just get the installed theme by looking at some file like .config/hypr/hyprtheme/meta.json
//But I still need the install thingy. I actually dont need `Theme` struct for
// the installed thing. It's just an installed theme which can get uninstalled.
// But what about updating? hmmm
// Ez, just look up what is installed, create the corresponding Theme struct, exec the update method,
// and then install method

impl Theme {
    /// Has an optional install_directory argument, as Hyprland allows for custom config paths
    pub fn install(&self, install_directory: Option<PathBuf>) -> Result<String, anyhow::Error> {
        // wut is this
        //
        let install_dir =
            install_directory.unwrap_or(expanduser(&DEFAULT_INSTALL_PATH).context(format!(
                "Failed to expand home directory for the default install path: {}",
                DEFAULT_INSTALL_PATH
            ))?);

        // standardize theme name
        // TODO: check theme name during installation, instead here
        let theme_name = self.meta.name.to_lowercase().replace(" ", "_");

        let theme_dir = install_dir.join(&theme_name);

        // check if theme is already installed
        // Why? Just install it again. User might want that, if he made changes to the files he might want to reset them.
        //  Easier for us
        // if theme_dir.exists() {
        //     println!("Theme {} is already installed", &self.meta.name);
        //     return Ok(());
        // }

        println!(
            "Installing theme {} to {}\n",
            &self.meta.name,
            theme_dir
                .to_str()
                .expect("Failed to install: Theme directory contains Non-UTF8 characters.")
        );
        // This stuff is for downloading the theme
        // It might need to be its own function
        // Probably its own function
        //
        // Now this is what I want:
        // Check if its download: Yes: Update? and install  | No: Download and install

        return Ok(install_dir
            .to_str()
            .to_owned()
            .expect("Install dir is not unicode "));
    }

    /// Not public, because you want to use the install method
    pub fn download(git_url: &String, branch: &String) -> Result<Self, anyhow::Error> {
        // Ahh, I need to first download the repo, before I can parse it
        // So install can not take `self`

        // How to get the theme name before downloading it?
        let repo_name = git_url.split("/").last();

        // clone repo
        let clone_cmd = format!(
            "cd {} && git clone --depth 1 --branch {} {}",
            (expanduser(DEFAULT_DOWNLOAD_PATH).context(format!(
                "Failed to expand default download path: {}",
                DEFAULT_DOWNLOAD_PATH
            ))?)
            .to_str()
            .context(format!("Download path contains non-unicode characters."))?,
            branch,
            git_url
        );

        std::process::Command::new("sh")
            .arg("-c")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg(&clone_cmd)
            .output()?;
    }

    // pub fn ensure_exists(&mut self) -> Result<(), anyhow::Error> {
    //     if self.path.exists() {
    //         return Ok(());
    //     }

    //     let parent = self
    //         .path
    //         .parent()
    //         .context("Failed to get parent path of the theme config.")?;
    //     if !parent.exists() {
    //         std::fs::create_dir_all(parent)?
    //     }

    //     std::fs::write(&self.path, "")
    //         .with_context(|| format!("Failed to write config to {}", self.path.display()))
    // }

    // TODO whats the point, we just install it again if the user really wants to lol
    // async fn is_installed(&self) -> Result<bool, anyhow::Error> {}

    /// Create a theme struct from a hyprtheme repo.
    ///
    /// Like this a themes can be loaded into memory, queried
    pub async fn from_directory(path: &PathBuf) -> Result<Self, anyhow::Error> {
        // The default locations of the hyprtheme.toml config
        let locations = vec!["./hyprtheme/hyprtheme.toml", "./hyprtheme.toml"];

        // Lets keep the errors by not using a find_map in case there are others reasons why the config cannot get accessed
        let config_string = locations
            .iter()
            .map(|location| {
                let config_path = path.join(location);
                return fs::read_to_string(&config_path).map_err(|error| {
                    anyhow::Error::from(error).context(format!(
                        "Failed to read out file at {}",
                        &config_path
                            .to_str()
                            .expect("Provided path has non-unicode characters ")
                    ))
                });
            })
            .collect::<Result<String, anyhow::Error>>()?;

        // This might need more work. Not all data can be read out from the string
        // I would like to add the theme path, too
        let config: Theme = toml::from_str(&config_string)?;

        // Lets prevent weirdly named themes, like "ФΞд฿Ŀ"
        if !config
            .meta
            .name
            .bytes()
            .all(|character| matches!(character, b'a'..=b'z' | b'0'..=b'9'))
        {
            return Err(anyhow!("Filename contains invalid characters. Only latin letters and numbers are allowed. "));
        }

        return Ok(config);
    }

    /// TODO Maybe we just want to simply uninstall the hyprtheme and not one theme.
    /// Would make more sense tbh
    // pub fn uninstall(&self) -> Result<self, String> {
    //     let install_dir = self.path.unwrap_or(
    //         expanduser("~/.config/hypr/themes").expect("Failed to expand the theme directory"),
    //     );

    //     let theme_dir = install_dir.join(&theme_name);

    //     // check if theme is already installed
    //     if !theme_dir.exists() {
    //         return Err(format!("Theme {} is not installed", &self.meta.name));
    //     }

    //     println!(
    //         "Uninstalling theme {} from {}",
    //         &self.meta.name,
    //         theme_dir.to_str().unwrap()
    //     );

    //     // delete dir
    //     match std::fs::remove_dir_all(theme_dir) {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(e.to_string()),
    //     }
    // }

    pub fn update(&self, install_dir: Option<PathBuf>) -> Result<(), String> {
        let install_dir = install_dir.unwrap_or(expanduser("~/.config/hypr/themes").unwrap());

        //standardize theme name
        let theme_name = self.meta.name.to_lowercase().replace(" ", "_");

        let theme_dir = install_dir.join(&theme_name);

        // check if theme is already installed
        if !theme_dir.exists() {
            return Err(format!("Theme {} is not installed", &self.meta.name));
        }

        println!(
            "Updating theme {} in {}",
            &self.meta.name,
            theme_dir.to_str().unwrap()
        );

        // delete dir
        match std::process::Command::new("sh")
            .arg("-c")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(&theme_dir)
            .arg("git pull")
            .output()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

// display
impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta.name,)
    }
}

#[derive(Debug, Deserialize)]
struct ThemeMeta {
    name: String,
    description: String,
    version: String,
    author: String,
    /// Git repository of the theme
    git: String,
    /// Entry point of the hyprtheme Hyprland config file.
    /// By default `./hypr/hyprtheme.config` or `./.hyprtheme/hyprtheme.config`
    entry: String,
    branch: String,
}

impl ThemeMeta {
    pub fn name(&self) -> String {
        //standardize theme name
        return self.name.to_lowercase().replace(" ", "_");
    }
}

/// Configuration how to move dot files to their locations
#[derive(Debug, Deserialize)]
struct ConfigFilesMeta {
    /// The default is `~/.config` and will be used if not set.
    root: String,
    /// Which dirs and files to ignore. Useful to ignore your own installer for example.
    /// By default `[ ".hyprtheme/", "./*\.[md|]", "LICENSE", ".git*" ]` is always ignored
    /// You can ignore everything with ['**/*'] and only `include` the dirs/files which you want
    ignore: Vec<String>,
    include: Vec<String>,
    //
    // Define how to move files from a to b.
    // Not sure how to do it in a nice way though
    //# manual_moves = []
}

#[derive(Debug, Deserialize)]
struct LifeTimeConfig {
    /// Gets run after the theme got installed. Usually to restart changed apps
    /// Default path: .hyprtheme/setup.sh - If found it will run it, even if not specified
    setup: String,
    /// Gets run after the theme got uninstalled. Usually to kill started apps
    /// Default path: .hyprtheme/cleanup.sh - If found it will run it, even if not specified
    cleanup: String,
}

/// Data for an optional extra configurations, like an optional workspaces setup
/// User can install them or not
#[derive(Debug, Deserialize)]
struct ExtraConfig {
    /// The name of the extra configuration.
    /// For example: `workspaces` (theme author also provides his own workspace setup, which might interfer with the users one)
    name: String,
    /// Path to the hyprlang `X.conf` which will if installed sourced to the hyprtheme.conf of the user
    path: String,
    /// Gets displayed to the user. Describes what this is
    description: Option<String>,
}
