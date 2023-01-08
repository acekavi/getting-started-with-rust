fn main() {
    let num = 2;
    if num > 5{
        println!("True");
    }else {
        println!("False");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
