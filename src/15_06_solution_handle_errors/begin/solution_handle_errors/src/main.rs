use rand::random_range;
use std::io;

fn main() {
    let secret_number = random_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(value) => value, // success
                Err(_) => {
                    println!("\nFailed to parse input. Guess again:");
                    continue;
                }
            },
            Err(_) => {
                println!("\nFailed to read input. Guess again:");
                continue;
            }
        };

        if guess > secret_number {
            println!("\n{guess} is too high! Guess lower:");
        } else if guess < secret_number {
            println!("\n{guess} is too low! Guess higher:");
        } else {
            println!("\nYou got it! The secret number was {secret_number}.");
            break;
        }
    }
}
