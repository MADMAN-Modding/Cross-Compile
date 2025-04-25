use std::path::Path;
use std::process::Command;

use colored::Colorize;

use crate::common::set_working_dir;

pub fn build_code(dir: &str, mut args: &str) -> Result<String, String> {
    let _ = set_working_dir(Path::new(dir));

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
            return Err(format!("{}{}", "error building".to_string(), reader));
            }
        }
        Err(e) => return Err(format!("Build Error: {}", e)),
    }

    return Ok("Build Success".to_string());
}


fn get_build_buf_reader(args: &str) -> Result<String, String> {
    let mut base_args = vec!["build"];

    if !args.contains("--debug") {
        base_args.extend(args.split_whitespace());
    }

    let command = Command::new("cargo")
        .args(&base_args)
        .output()
        .map_err(|err| format!("Build Error: {}", err))?;

    Ok(String::from_utf8_lossy(&command.stderr).to_string())
}