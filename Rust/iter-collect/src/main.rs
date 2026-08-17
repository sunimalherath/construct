fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        "blue".to_string(),
    ];

    let upper_cased = to_uppercase(&colors);

    println!("{:#?}", upper_cased);
    println!("{:#?}", to_uppercase_two(&colors));
    println!("{:#?}", to_uppercase_three(&colors));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect() 
    // since collect() is the last part of the statement, it knows that its responsible for the
    // return value from this function - so, it collects the values into a Vec<String>
}

fn to_uppercase_two(elements: &[String]) -> Vec<String> {
    // explicitly stating to collect to a Vec<String>
    elements.iter().map(|el| el.to_uppercase()).collect::<Vec<String>>()
}

fn to_uppercase_three(elements: &[String]) -> Vec<String> {
    // explicitly stating to collect to a Vec<String>
    let upcased: Vec<String> = elements.iter().map(|el| el.to_uppercase()).collect();

    upcased
}
