fn main() {
    let mut x = 5;
    println!("Value of x is {x}");
    x = 10;
    println!("Value of x is {x}");

    const SEC_IN_H:u16 = 60 * 60;
    println!("Seconds in an hour: {SEC_IN_H}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("number of spaces: {spaces}");
}
