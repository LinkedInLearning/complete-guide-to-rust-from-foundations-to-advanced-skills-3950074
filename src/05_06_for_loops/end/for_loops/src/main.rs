fn main() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate() {
        println!("index is {index} : item is {item}");
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {number}");
    }
}
