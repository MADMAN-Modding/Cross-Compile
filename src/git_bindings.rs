use std::{
    env::set_current_dir,
    io::{self},
    path::Path,
    process::Command,
};

use colored::Colorize;

/**
 * Pulls the code of the supplied path
 * Will return ```Ok(())``` as long as there is no error
 * Fetches data
 * Gets the current head
 * Asks user to pull data
 * Error is a ```String```
 */
pub fn pull_repo(directory: &Path) -> Result<(), String> {
    
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


    // Fetches repository
    let _fetch = Command::new("git")
        .arg("fetch")
        .output()
        .map_err(|err| format!("Failed to Fetch Updates {}", err));

    // Get the current branch name
    let branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|err| format!("Failed to get current branch: {}", err))?;

    // Takes the name of the branch from the output of the command
    let branch_name = String::from_utf8_lossy(&branch.stdout).trim().to_string();

    let log = Command::new("git")
        .args(["log", "--oneline", &format!("HEAD..origin/{}", branch_name)])
        .output()
        .map_err(|err| format!("Failed to get Status {}", err))?;

    // Outputs the stdout of the command
    let log_output = String::from_utf8_lossy(&log.stdout);

    // If the fetch is empty than everything is up to date already
    if log_output.is_empty() {
        println!("{}", "Repository is up to date\nNo data will be pulled".yellow().bold());

        return Ok(());
    }

    // Outputs the changes
    println!("\n{}", log_output.blue());

    // Stores user input
    let mut user_input: String = String::new();

    println!("Would you like to pull with the above changes? [Y/n]");

    // Gets user input
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read from stdin");

    /* Makes a bool from the user's decision
    Will be true unless 'n' is supplied */
    let decision: bool = !user_input.to_lowercase().as_str().contains("n");

    if decision {
        // Makes a command for pulling the repo
        let pull = Command::new("git")
            .args(["-c", "color.ui=always", "pull"])
            .output()
            .map_err(|err| format!("Failed to pull changes {}", err))?;

        // Outputs the pulled data
        println!("{}", String::from_utf8_lossy(&pull.stdout).blue());

        // If the pull fails, return an error
        if pull.status.success() {
            return Ok(());
        } else {
            return Err("Failed pulling latest changes".to_string());
        }
    } else {
        println!("{}", "Will not pull repository".blue())
    }

    return Ok(());
}
