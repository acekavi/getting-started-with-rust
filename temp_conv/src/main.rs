use std::io;

fn main() {
    let mut choice = String::new();

    println!("Enter the celsius value you need to convert to farenheit -> ");
    io::stdin()
        .read_line(&mut choice)
        .expect("Please enter a valid number!");

    let value: u32 = choice
        .trim()
        .parse()
        .expect("Please enter a valid integer!");

    let value = value * (9 / 5) + 32;
    println!(
        "---------------------------------------------\n{} celsius is {value} in farenheit.",
        choice.trim()
    );
}
