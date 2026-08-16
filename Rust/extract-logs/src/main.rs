use std::fs;
use std::io::Error;

fn main() {
    let error_logs = get_logs("logs.txt", "ERROR");

    println!("{:?}", error_logs);
    // println!("{}", error_logs.iter().len());
}

fn get_logs(file_name: &str, log_type: &str) -> Result<Vec<String>, Error> {
    let full_content = fs::read_to_string(file_name)?;

    println!("all lines: {:?}", full_content);

    let all_lines = full_content.split("\n");

    let mut extracted_text: Vec<String> = Vec::new();

    for line in all_lines {
        if line.starts_with(log_type) {
            extracted_text.push(line.to_string());
        }
    }

    Ok(extracted_text)
}
