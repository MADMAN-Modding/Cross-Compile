use std::{env::{self}, path::Path};

use colored::Colorize;
use git_bindings::pull_repo;

pub mod building {
    mod build_rust;
}

pub mod git_bindings;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let result: Result<String, String> = match args[1].as_str() {
            "--setup" => {setup_dir(&args[2])}
            "--update" => {update_dir(&args[2])}
            _ => {Err("Option Not Found".to_string())}
        };

        if result.is_ok() {
            println!("{}", result.unwrap().green().bold())
        } else {
            println!("Failed to execute {}\nError: {}", args[1].red(), result.err().unwrap().red())
        }
    } else {
        println!("{}", "No cli args found".red());
    }
}

fn setup_dir(directory: &str) -> Result<String, String> {
    if Path::new(&directory).exists() {
        return Ok("Directory Found".to_string());
    }

    return Err("Directory Not Found".to_string())
}

fn update_dir(directory: &str) -> Result<String, String> {
    let dir = Path::new(directory);
    let git_dir = dir.join(".git").as_path().to_owned();

    if git_dir.exists() {

        let result: Result<(), String> = pull_repo(&dir);
        
        match result.is_ok() {
            true => return Ok("Updating Complete".to_string()),
            false => return Err(format!("Updating Failed: {}", result.err().unwrap()))
        };
    }

    return Err("Git dir not found".to_string());

}