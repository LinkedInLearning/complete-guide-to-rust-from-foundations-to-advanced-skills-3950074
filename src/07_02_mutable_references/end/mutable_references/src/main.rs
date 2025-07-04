fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {rocket_fuel} and length is {length}");
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {propellant}...");
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}
