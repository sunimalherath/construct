fn print_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn main() {
    let v = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elements(&v);
}
