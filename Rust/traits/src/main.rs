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
