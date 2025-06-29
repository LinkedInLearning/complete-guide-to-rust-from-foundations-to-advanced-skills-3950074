use rand::random_range;
use std::io;

fn main() {
    let secret_number = random_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line.");
        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");

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
