fn main() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel_length = process_fuel(&rocket_fuel);
    println!(
        "rocket fuel is {} with length {}",
        rocket_fuel, rocket_fuel_length
    );
}

fn process_fuel(propellant: &String) -> usize {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    return length;
}
