#![allow(unused)]

struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan is driving");
    }
}

struct SUV;
impl LandCapable for SUV {
    fn drive(&self) {
        println!("SUV is driving");
    }
}

trait LandCapable {
    fn drive(&self);
}

/// Demonstrates dynamic dispatch
fn dyn_road_trip(vehicle: &dyn LandCapable) {
    vehicle.drive();
}

/// Demonstrates static dispatch
fn static_road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

#[cfg(test)]
mod test {
    use super::{dyn_road_trip, Sedan, SUV, static_road_trip};

    #[test]
    fn drives() {
        let sedan = Sedan {};
        dyn_road_trip(&sedan);
        static_road_trip(&sedan);
        
        let suv = SUV {};
        dyn_road_trip(&suv);
        static_road_trip(&suv);
    }
}
