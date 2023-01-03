use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    println!("Please enter your guess: ");

    let mut guess=String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let secret = rand::thread_rng().gen_range(1..=10);
    println!("Secret number is: {secret}")
}
