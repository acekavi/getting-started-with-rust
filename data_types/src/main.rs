fn main() {
    // 'u' means unsigned(Numbers that will always be positive)
    // 'i' means signed(with the + or -)

    // u8 can store values from (0 to 2^7 - 1) = 0 to 2550
    // i8 can store values from -(2^7) to 2^7 - 1) = -127 to 127
    let x: i8 = 1; // f64

    let y: i8 = 125; // f32
    println!("{}", x + y);
}