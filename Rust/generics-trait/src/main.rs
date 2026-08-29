trait Vehicle {
    // abstract method
    fn start(&self);

    // default method
    fn stop(&self) {
        println!("vehicle stoped");
    }
}

struct Car {}

// Car needs to implement all the abstract method of the trait.
// No necesary to implement default methods as those alreay have a default behaviour.
// Also, if needed default behaviour can be overridden in the implementation. 
impl Vehicle for Car {
    fn start(&self) {
        println!("car started");
    }
}

fn start_and_stop<T: Vehicle>(vehicle: T) {
    vehicle.start();
    vehicle.stop();
}

fn main() {
    let l663 = Car {};

    start_and_stop(l663);
}
