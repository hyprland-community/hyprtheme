use super::helper::parse_path;
use crate::theme::{self, create_theme_id, installed::InstalledTheme, online, saved};
use anyhow::Result;
use clap::Parser;
use regex::RegexBuilder;
use std::path::PathBuf;

#[derive(Parser)]
pub struct InstallArgs {
    /// Either:
    /// - Name of a theme featured on www.hyprland-community.org/hyprtheme/browse
    /// - Git repository: https://github.com/hyprland-community/hyprtheme
    /// - Github short-form: author/repo-name
    #[arg(short,long,value_parser=ThemeName::parse)]
    pub name: ThemeName,

    /// The branch of the repository to install
    #[arg(short, long)]
    pub branch: Option<String>,

    /// The data directory of Hyprtheme by default "~/.local/share/hyprtheme/"
    /// The theme will be saved in the sub-directory "themes"
    #[arg(short,long,default_value="~/.local/share/hyprtheme/",value_parser=parse_path)]
    pub data_dir: PathBuf,

    /// The path to the the Hyprland config directory, where the theme will be installed to.
    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub hypr_dir: PathBuf,
}

impl InstallArgs {
    pub async fn install(&self) -> Result<InstalledTheme> {
        struct GitUrlBranch {
            pub url: String,
            pub branch: Option<String>,
        }

        let git_data: GitUrlBranch = match &self.name {
            ThemeName::Featured(theme) => {
                // There doesn't seem to be a way to determine if a featured theme is already installed
                // only by it's name, as themes from other repos can have the same name

                // TODO we need to ban featured themes with the same names or handle this case with a prompt again

                let found_theme = online::find_featured(&theme)
                    .await
                    .expect("Failed to fetch featured theme")
                    .map(|theme| GitUrlBranch {
                        url: theme.repo,
                        branch: theme.branch,
                    })
                    .expect("Failed to find theme by provided name.");

                GitUrlBranch {
                    url: found_theme.url,
                    branch: self.branch.clone().or(found_theme.branch),
                }
            }

            ThemeName::Github((author, repo)) => GitUrlBranch {
                url: "git@github.com:".to_string() + &author + "/" + &repo + ".git",
                branch: self.branch.clone(),
            },

            ThemeName::Git(github_string) => GitUrlBranch {
                url: github_string.clone(),
                branch: self.branch.clone(),
            },
        };

        let saved_theme = saved::find_saved(
            &create_theme_id(&git_data.url, git_data.branch.as_deref()),
            Some(&self.data_dir),
        )
        .await
        .unwrap_or({
            println!("Failed to lookup saved themes! Downloading theme to be safe...");
            None
        });

        let saved_theme = match saved_theme {
            Some(saved) => saved,
            None => {
                let downloaded = theme::online::download(
                    &git_data.url,
                    git_data.branch.as_deref(),
                    Some(&self.data_dir),
                )
                .await
                .expect("Failed to download theme");
                println!("Downloaded theme.");
                downloaded
            }
        };

        saved_theme.install(Some(&self.hypr_dir)).await
    }
}

#[derive(Clone)]
pub enum ThemeName {
    /// Name of a theme featured on the Hyprtheme website
    Featured(String),
    /// Repository of a theme, like: git@github.com:hyprland-community/hyprtheme.git
    Git(String),
    /// Short version of a Github repository:
    /// author/repo-name
    ///
    /// Holds a vector of (author, repo-name)
    Github((String, String)),
}
impl ThemeName {
    pub fn parse(string: &str) -> Result<Self> {
        let github_regex = RegexBuilder::new(
            r"^[a-z\d](?:[a-z\d]|-(?=[a-z\d])){0,38}\/[a-z\d](?:[a-z\d]|-(?=[a-z\d]))*$",
        )
        .case_insensitive(true)
        .build()
        .unwrap();
        if github_regex.is_match(string) {
            let (name, repo) = string
                .split_once("/")
                .expect("Git repo regex failed. Could not split at /");

            return Ok(Self::Github((name.to_owned(), repo.to_owned())));
        }

        let git_repo_regex = RegexBuilder::new(
            r"^((git|ssh|http(s)?)|(git@[\w\.-]+))(:(//)?)([\w\.@\:/\-~]+)(\.git)(/)?$",
        )
        .case_insensitive(true)
        .build()
        .unwrap();
        if git_repo_regex.is_match(string) {
            return Ok(Self::Git(string.to_owned()));
        }

        // We cannot fetch theme names here,
        // as this would be async and Clap doesnt like that
        // so we just assume it's a featured theme name
        Ok(Self::Featured(string.to_owned()))
    }
}
