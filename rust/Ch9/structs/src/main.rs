#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let mut my_shuttle: Shuttle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835812.0,
    };

    println!("name is {}", my_shuttle.name);

    my_shuttle.name = String::from("Atlantis");
    println!("vehicle is {:?}", my_shuttle);

    let my_other_shuttle: Shuttle = Shuttle {
        name: String::from("Discovery"),
        ..my_shuttle
    };

    println!("other vehicle is {:?}", my_other_shuttle);
}
