fn main() {
    let countdown = [3, 2, 1, 0];
    match countdown.get(3) {
        Some(0) => println!("BLASTOFF!"),
        _ => (),
    }
}
