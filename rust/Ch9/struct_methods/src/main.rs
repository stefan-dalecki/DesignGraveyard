struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavor"),
        crew_size: 7,
        propellant: 0.0,
    };

    let vehicle_name = vehicle.get_name();
    println!("Vehicle name is {}", vehicle_name);

    println!("Starting fuel is {}", vehicle.propellant);

    vehicle.add_fuel(100.0);

    println!("Ending fuel is {}", vehicle.propellant);
}
