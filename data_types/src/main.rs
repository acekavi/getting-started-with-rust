use std::io;

fn main() {
    // // 'u' means unsigned(Numbers that will always be positive)
    // // 'i' means signed(with the + or -)

    // // u8 can store values from (0 to 2^7 - 1) = 0 to 2550
    // // i8 can store values from -(2^7) to 2^7 - 1) = -127 to 127
    // let x: i8 = 1; // f64

    // let y: i8 = 125; // f32
    // println!("{}", x + y);

    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // // remainder
    // let remainder = 43 % 5;
    // println!("-------------------------------------------\nArithmatic expressions");
    // println!("additiion {sum}, difference {difference}, product {product}, quotient {quotient}, truncated {truncated}, remainder {remainder}");

    // //Tuples
    // println!("-------------------------------------------\nTuples");
    // let x = (500, 6.4, 1);

    // let _five_hundred = x.0;

    // let _six_point_four = x.1;
    // let _one = x.2;
    // println!("{x:?}");

    // //Arrays
    // // More like java arrays. Same type and a definite array length
    // println!("-------------------------------------------\nArrays");
    // let months: [&str; 7] = ["jan", "feb", "mar", "april", "may", "jun", "jul"];
    // println!("{months:?}");

    // let num_arr = [5; 3];
    // let ind_num = num_arr[1];
    // println!("{ind_num}");
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}