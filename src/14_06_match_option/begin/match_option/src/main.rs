fn main() {
    let countdown = [3, 2, 1, 0];
    let number = countdown.get(4);
    let number = number.unwrap_or(&0) + 1;
    println!("number is {number:?}");
}
