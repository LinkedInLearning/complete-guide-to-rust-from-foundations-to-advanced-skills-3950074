fn main() {
    let my_number = 3u8;

    let result = match my_number {
        0 => "zero",
        _ => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{my_number} did not match");
            "something else"
        }
    };
    println!("result is {result}");
}
