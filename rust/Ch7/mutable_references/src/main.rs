fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let rocket_fuel_length = process_fuel(&mut rocket_fuel);
    println!(
        "rocket fuel is {} with length {}",
        rocket_fuel, rocket_fuel_length
    );
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}
