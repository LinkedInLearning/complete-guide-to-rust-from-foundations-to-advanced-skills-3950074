use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }
    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line == search_name {
            println!("{search_name} is in the list.");
            return;
        }
    }

    println!("{search_name} was NOT found in the list.");
}
