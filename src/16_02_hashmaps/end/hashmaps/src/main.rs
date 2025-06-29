use std::collections::HashMap;

fn main() {
    let mut missions_flown = HashMap::new(); // space missions
    missions_flown.insert("Armstrong", 2); // Neil Armstrong
    missions_flown.insert("Kelly", 5); // Scott Kelly
    missions_flown.insert("Kim", 1); // Jonny Kim
    missions_flown.insert("Kim", 2);
    missions_flown.entry("Stone").or_insert(3);
    let jonny = missions_flown.entry("Kim").or_insert(0);
    *jonny += 1;
    println!("missions_flown is {missions_flown:?}");

    let kim_missions = missions_flown.get("Kim");
    println!("kim_missions is {kim_missions:?}");
}
