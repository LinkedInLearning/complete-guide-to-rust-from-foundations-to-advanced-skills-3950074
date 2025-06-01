use rand;

fn main() {
    let a = rand::random::<f64>();
    println!("a is {a}");

    let b = random();
    println!("b is {b}");

    let c = rand::random_range(1..11);
    println!("c is {c}");
}

fn random() -> f64 {
    42.0
}
