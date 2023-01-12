use std::io;

fn main() {
    let mut user_input = String::new();
    println!("How many terms?");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Please enter a valid value!");

    let nterms: i32 = user_input.trim().parse().expect("PLease enter an integer!");

    let mut num_1: i32 = 0;
    let mut num_2: i32 = 1;

    let mut count: i32 = 0;

    if nterms <= 0 {
        println!("Please enter a positive integer");
    } else {
        print!("Fibonacci sequence: ");
        while count < nterms {
            print!("{num_1},");
            let nth = num_1 + num_2;
            num_1 = num_2;
            num_2 = nth;
            count += 1;
        }
    }
    println!("\n");
}
