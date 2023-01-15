fn main() {
    let mut text = String::from("Hello");
    text.push_str(" World!");

    println!("{text}");

    //"text" is still valid
    let x = 5;
    let y = x;
    println!("{x}, {y}");

}   //end of "text" scope

