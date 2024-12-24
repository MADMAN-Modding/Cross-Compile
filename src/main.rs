use std::{
    env::{self},
    fs::create_dir_all,
    path::Path,
};

use colored::Colorize;
use git_bindings::{clone_repo, pull_repo, remove_repo};

pub mod building {
    mod build_rust;
}

pub mod git_bindings;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let result: Result<String, String> = match args[1].as_str() {
            "-s" | "--setup" => setup_dir(&args[2]),
            "-u" | "--update" => update_dir(&args[2]),
            "-c" | "--clone" => clone_dir(&args[2], &args[3]),
            "-r" | "--remove" => remove_git_dir(&args[2]),
            _ => Err("Option Not Found".to_string()),
        };

        if result.is_ok() {
            println!("{}", result.unwrap().green().bold())
        } else {
            println!(
                "Failed to execute {}\nError: {}",
                args[1].red().bold(),
                result.err().unwrap().red().bold()
            )
        }
    } else {
        println!("{}", "No cli args found".red());
    }
}

fn setup_dir(directory: &str) -> Result<String, String> {
    if Path::new(&directory).exists() {
        return Ok("Directory Found".to_string());
    }

    return Err("Directory Not Found".to_string());
}

fn update_dir(directory: &str) -> Result<String, String> {
    if check_git_dir(directory) {
        let result: Result<(), String> = pull_repo(Path::new(directory));

        match result.is_ok() {
            true => return Ok("Updating Complete".to_string()),
            false => return Err(format!("Updating Failed: {}", result.err().unwrap())),
        };
    }

    return Err("Git dir not found".to_string());
}

fn clone_dir(directory: &str, repository: &str) -> Result<String, String> {
    let dir = Path::new(directory);

    let _ = create_dir_all(dir);

    let result = clone_repo(dir, repository);

    match result.is_ok() {
        true => return Ok("Directory Cloning Complete".to_string()),
        false => return Err(format!("Cloning Failed: {}", result.err().unwrap())),
    }
}

fn check_git_dir(directory: &str) -> bool {
    let dir = Path::new(directory);
    let git_dir = dir.join(".git").as_path().to_owned();

    return git_dir.exists();
}

fn remove_git_dir(directory: &str) -> Result<String, String> {
    if check_git_dir(directory) {
        let result = remove_repo(Path::new(directory));

        println!("{}", format!("Deleting: {}", directory).blue());

        match result.is_ok() {
            true => return Ok("Repository Deleted".to_string()),
            false => {
                return Err(format!(
                    "Unable to delete repository: {}",
                    result.err().unwrap()
                ))
            }
        }
    }

    return Err("Directory does not contain a git directory or does not exist".to_string());
}
