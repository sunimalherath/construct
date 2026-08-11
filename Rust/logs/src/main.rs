use std::{fs, io::read_to_string};

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
    // using the expect method of the Result enum instead of the match statements
    let text = fs::read_to_string("logs.txt").expect("failed to read the logs.txt");
    let error_log = extract_errors(text.as_str());
    fs::write("errors.txt", error_log.join("/n")).expect("failed to write to the errors file");

    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            let error_log = extract_errors(text.as_str());

            match fs::write("errors.txt", error_log.join("\n")) {
                Ok(..) => println!("wrote to errors.txt"),
                Err(msg) => println!("failed to write to errors.txt due to: {}", msg),
            }
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}
