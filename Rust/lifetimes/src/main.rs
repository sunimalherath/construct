fn main() {
    let matrix_phrase = "there is no spoon";
    let inception_phrase = "dream within a dream";

    println!("{}", longest_phrase(matrix_phrase, inception_phrase));
}

fn longest_phrase<'a>(phrase_a: &'a str, phrase_b: &'a str) -> &'a str {
    if phrase_a.len() >= phrase_b.len() {
        phrase_a
    } else  {
        phrase_b
    } 
}
