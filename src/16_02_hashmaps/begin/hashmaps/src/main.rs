use std::collections::HashMap;

fn main() {
    let mut missions_flown = HashMap::new(); // space missions
    missions_flown.insert("Armstrong", 2); // Neil Armstrong
    missions_flown.insert("Kelly", 5); // Scott Kelly
    missions_flown.insert("Kim", 1); // Jonny Kim
    println!("missions_flown is {missions_flown:?}");
}
