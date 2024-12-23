use std::{env::{self}, path::Path};

use git_bindings::pull_repo;

pub mod building {
    mod build_rust;
}

pub mod git_bindings;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !args[1].is_empty() {
        let result: Result<&str, &str> = match args[1].as_str() {
            "--setup" => {setup_dir(&args[2])}
            "--update" => {update_dir(&args[2])}
            _ => {Err("Option Not Found")}
        };

        if result.is_ok() {
            println!("{}", result.unwrap())
        } else {
            println!("Failed to execute {}", args[1])
        }
    } else {
        println!("No cli args found");
    }
}

fn setup_dir(directory: &str) -> Result<&str, &str> {
    if Path::new(&directory).exists() {
        return Ok("Directory Found");
    }

    return Err("Directory Not Found")
}

fn update_dir(directory: &str) -> Result<&str, &str> {
    let dir = Path::new(directory);
    let git_dir = dir.join(".git").as_path().to_owned();

    // Removes dir variable cause it's no longer needed
    drop(dir.to_owned());

    println!("{}", &git_dir.to_path_buf().to_str().unwrap());

    if git_dir.exists() {

        let result: bool = pull_repo(&git_dir);
        
        match result {
            true => return Ok("Pull Complete"),
            false => return Err("Pull Failed")
        };
    }

    return Err("Git dir not found");

}