/*
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let number = Some(13);
    if let Some(13) = number {
        if Some(i32) == type_of(number) {
            println!("i32");
        }
        println!("thirteen");
        println!("type is {}", type_of(number));
    }
}
*/

fn main() {
    let countdown = [5, 4, 3, 2, 1, 0];
    let ready_for_launch = true;
    if let Some(1) = countdown.get(4) {
        if ready_for_launch {
            println! ()
        }
    } else {
        return;
    }
}