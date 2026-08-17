fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        "blue".to_string(),
    ];

    let upper_cased = to_uppercase(&colors);

    println!("{:#?}", upper_cased);
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}
