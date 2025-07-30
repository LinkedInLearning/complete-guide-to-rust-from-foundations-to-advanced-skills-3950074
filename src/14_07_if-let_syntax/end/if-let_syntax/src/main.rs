fn main() {
    let countdown = [3, 2, 1, 0];
    if let Some(0) = countdown.get(3) {
        println!("BLASTOFF!");
    }
}
