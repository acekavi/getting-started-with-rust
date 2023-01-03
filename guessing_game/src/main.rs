use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret = rand::thread_rng().gen_range(1..=10);
    println!("Secret number is: {secret}");

    loop {
        println!("Please enter your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : i16 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too Small!");
                continue;
            },
            Ordering::Greater => {
                println!("Too Big!");
                continue;
            },
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        };
    }
    
}