use std::fs;

fn extract_errors(text: &str) -> Vec<&str> {
    let all_text = text.split("\n");

    let mut err_text = vec![];

    for line in all_text {
        if line.starts_with("ERROR") {
            err_text.push(line);
        }
    }

    err_text
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            let error_log = extract_errors(text.as_str());
            println!("{:#?}", error_log);
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}
