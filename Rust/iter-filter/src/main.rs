fn main() {
    let words = vec![
        String::from("the matrix has you"),
        String::from("there is no spoon"),
        String::from("follow the white rabbit"),
    ];

    println!("{:#?}", find_pattern(&words, "cat", "dejavu"));
    println!("{:#?}", find_pattern(&words, "ab", "dejavu"));
}

fn find_pattern(elements: &[String], pattern: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(pattern))
        .map_or(String::from(fallback), |el| el.to_string())
}
