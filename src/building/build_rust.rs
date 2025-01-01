use std::path::Path;
use std::process::Command;

use colored::Colorize;

use crate::common::set_working_dir;

pub fn build_code(dir: &str, mut args: &str) -> Result<String, String> {
    let _ = set_working_dir(Path::new(dir));

    println!("{}", dir);

    if args.is_empty() {
        eprintln!(
            "{}",
            "No build arguments provided, defaulting to \"--release\""
                .yellow()
                .bold()
        );

        args = "--release";
    }

    let reader = get_build_buf_reader(args);

    match reader {
        Ok(reader) => {
            if reader.to_lowercase().contains("error") {
                return Err("error building".to_string());
            }
        }
        Err(e) => return Err(format!("Build Error: {}", e)),
    }

    return Ok("Build Success".to_string());
}

fn get_build_buf_reader(args: &str) -> Result<String, String> {
    println!("{}", args);

    let command = Command::new("cargo")
        .args(["build", args])
        .output()
        .map_err(|err| format!("Build Error: {}", err))?;


    // if stderr.is_some() {
    //     return Err("Command Failed".to_string());
    // }

    // let reader = BufReader::new(stdout);

    Ok(String::from_utf8_lossy(&command.stderr).to_string())
}
