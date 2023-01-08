fn main() {
    // Can equal return values of a function to a immutable variable
    let r_value = another_function(45);
    println!("{r_value}");
}

fn another_function(mut value: i32) -> i32{
    value = value + 6;
    // just the value without a return statement acts as an expression.
    value

    // can also return the value as
    // return value;
}