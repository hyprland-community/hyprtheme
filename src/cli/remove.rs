use super::helper::parse_path;
// use crate::theme::saved::{self, SavedTheme};
use anyhow::{anyhow, Result};
use clap::Parser;
use promptuity::prompts::{Select, SelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Promptuity, Term};
use std::path::PathBuf;

#[derive(Parser,Clone)]
pub struct RemoveArgs {
    /// The name of the theme to remove from the data directory
    #[arg(short, long)]
    pub theme_name: String,

    /// The data directory of Hyprtheme, by default in `~/.local/share/hyprtheme/`
    #[arg(short, long, value_parser=parse_path)]
    pub data_dir: Option<PathBuf>,
}

// impl RemoveArgs {
//     pub async fn remove(&self) -> Result<()> {
//         // If we have 3 themes installed, hosted on Github, Gitlab and some self-hosted thingy,
//         // and all are called Gruvbox and are from different users, all called Foo.
//         // Now we want to remove Gruvbox. But which one?
//         // So let's just prompt if tere are multiple themes with the same name

//         let themes = saved::get_all(self.data_dir.as_ref())
//             .await
//             .expect("Failed to lookup saved themes.");

//         let mut matched_themes: Vec<SavedTheme> = themes
//             .into_iter()
//             .filter(|theme| theme.config.meta.name.to_lowercase() == self.theme_name.to_lowercase())
//             .collect();

//         let matched: Option<SavedTheme> = match matched_themes.len() {
//             0 => None,
//             1 => Some(matched_themes.pop().unwrap()),
//             _ => Some(
//                 prompt_theme_selection(matched_themes)
//                     .expect("Failed to prompt for theme selection"),
//             ),
//         };

//         match matched {
//             Some(theme) => theme.remove().expect("Failed to remove theme"),
//             None => return Err(anyhow!("Could not find saved theme: {}", self.theme_name)),
//         }

//         return Ok(());
//     }
// }

// fn prompt_theme_selection(mut themes: Vec<SavedTheme>) -> Result<SavedTheme> {
//     let mut term = Term::default();
//     let mut theme = FancyTheme::default();
//     let mut p = Promptuity::new(&mut term, &mut theme);

//     p.with_intro(format!("Which {} to remove?", themes[0].config.meta.name))
//         .begin()?;

//     let id = p.prompt(
//         Select::new(
//             "Found multiple saved themes with the same name. Please select which to remove:",
//             themes
//                 .iter()
//                 .map(|theme| {
//                     SelectOption::new(
//                         format!(
//                             "By {} - {}",
//                             theme.config.meta.author, theme.config.meta.description
//                         ),
//                         theme.get_id(),
//                     )
//                 })
//                 .collect(),
//         )
//         .with_hint("Submit with Space or Enter.")
//         .as_mut(),
//     )?;
//     let index = themes
//         .iter()
//         .position(|theme| theme.get_id() == id)
//         .expect("Failed to use provided ID to select from themes");

//     Ok(themes.swap_remove(index))
// }
