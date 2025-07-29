/*
struct Shuttle {crew_size: u8}
struct Satellite {}

fn main() {
    //let spacecraft = Some(Shuttle{crew_size:4});
    let spacecraft = Some(Satellite{});
    match spacecraft {
        Some(ref Shuttle) => println!("Shuttle with {} crew members", 234),
        _ => ()
    }
}

*/
/*
fn main() {
    let number = Some(13);

    match number {
        Some(13) => println!("thirteen"),
        _ => (),
    }
}
*/

/*
fn main() {
    let countdown = [5, 4, 3, 2, 1];
    match countdown.get(4) {
        Some(1) => println!("one"),
        _ => ()
    };
}
*/
///*
fn main() {
    let countdown = [5, 4, 3, 2, 1, 0];
    if let Some(1) = countdown.get(4) {
        println!("one");
    }
}
//*/