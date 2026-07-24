trait Shape {
    fn calc_area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn calc_area(&self) -> f64 {
        3.14 * self.radius * self.radius 
    }
}

impl Shape for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle { width: 3.2, height: 4.5 };
    let c = Circle { radius: 2.4 };

    println!("Area of the Circle: {}", c.calc_area());
    println!("Area of the Rectangle: {}", r.calc_area());
}
