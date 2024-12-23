use std::{env::set_current_dir, io, path::Path, process::Command};

pub fn pull_repo(directory: &Path) -> bool {
    assert!(set_current_dir(&directory).is_ok());
    println!("Set working directory to {}", directory.display());

    // Command for git stuff
    let mut git = Command::new("git");

    // Fetches repository
    let fetch = &git.arg("fetch").output().expect("Failed to Fetch Updates");

    // Makes a command for pulling the repo
    let pull = git.arg("pull");

    // Outputs the stdout of the command
    let output = &fetch.stdout;

    // If the fetch is empty than everything is up to date already
    if String::from_utf8_lossy(&fetch.stdout) != "" {
        println!("{}", String::from_utf8_lossy(&output));
        drop(output.to_owned());
        drop(fetch.to_owned());

        // Stores user input
        let mut input: String = String::new();

        println!("Would you like to pull with the above changes? [Y/n]");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");

        let decision: bool = match input.to_lowercase().as_str() {
            "n" => false,
            _ => true,
        };

        println!("Decision was: {}", decision);

        if decision {
            pull.output().expect("Unable to pull repo");
        } else {
            println!("Will not pull repository")
        }

        return true;
    }

    println!("Repository is up to date\nNo data will be pulled");

    return false;
}
