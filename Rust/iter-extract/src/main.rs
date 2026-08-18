fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let extracted = extract_elements(&colors);

    println!("{:#?}", extracted);
}

fn extract_elements(elements: &Vec<String>) -> Vec<Vec<String>> {
    elements
        .into_iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}
