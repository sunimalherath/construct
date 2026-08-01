// trait Shape {
//     fn calc_area(&self) -> f64;
// }

// #[derive(Debug)]
// struct Circle {
//     radius: f64,
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// impl Shape for Circle {
//     fn calc_area(&self) -> f64 {
//         3.14 * self.radius * self.radius
//     }
// }

// impl Shape for Rectangle {
//     fn calc_area(&self) -> f64 {
//         self.width * self.height
//     }
// }

trait Printer {
    fn print_location(&self);
}

struct GeoLocation {
    latitude: String,
    longitude: String,
}

impl Printer for GeoLocation {
    fn print_location(&self) {
        println!("Long: {}, Lat: {}", &self.longitude, &self.latitude);
    }
}

fn main() {
    let my_location = GeoLocation {
        latitude: String::from("some lat"),
        longitude: String::from("some long"),
    };

    println!("{:?}", my_location.print_location());

    print_formatter(&my_location);

    print_formatter_simplyfied(&my_location);
}

fn print_formatter(item: &impl Printer) {
    println!("The current location:");
    item.print_location();
}

fn print_formatter_simplyfied<T: Printer>(item: &T) {
    println!("The current location from simplyfied:");
    item.print_location();
}
