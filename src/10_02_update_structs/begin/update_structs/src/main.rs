#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 1585000.0,
    };

    println!("vehicle is {vehicle:?}");
}
