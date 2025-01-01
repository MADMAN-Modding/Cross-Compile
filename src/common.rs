use std::{env::set_current_dir, path::Path};

use colored::Colorize;

pub fn set_working_dir(directory: &Path) -> Result<(), String> {
    // Handles errors setting the directory
    if let Err(err) = set_current_dir(directory) {
        return Err(format!(
            "{}: {}",
            "Failed to set working directory".red().bold(),
            err
        ));
    }

    println!(
        "{} {}",
        "Set working directory to".green().bold(),
        directory.display()
    );
    Ok(())
}
