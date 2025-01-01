use super::build_rust::build_code;

pub fn handle_input(input: Vec<String>) -> Result<String, String> {
    let result = match input[2].to_lowercase().as_str() {
        "rust" => build_code(&input[3], &input.get(4).unwrap_or(&"".to_string())),
        _ => return Err("Not a support language".to_string()),
    };

    result
}
