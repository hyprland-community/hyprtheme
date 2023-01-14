use colored::*;

use crate::cli::parse::Hyprtheme;

fn handle_error(e: &str) {
    println!("{}", e.red().bold());
}


