
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

fn road_trip(vehicle: &dyn LandCapable) {
    vehicle.drive();
}


#[cfg(test)]
mod test {
    use super::{road_trip, Sedan, SUV};

    #[test]
    fn drives() {
        let sedan = Sedan{};
        let suv = SUV{};

        road_trip(&sedan);
        road_trip(&suv);
    }
}
