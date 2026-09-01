mod basket;
mod stack;

use basket::Basket;
use stack::Stack;

fn main() {
    let b1 = Basket::new(String::from("first item"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let s1 = Stack::new(vec![String::from("stack item")]);

    println!("Hello, world!");
}
