fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(s1);

    println!("The length of '{}' is {}.", len.0, len.1);
}

fn calculate_length(mut s: String) -> (String, usize) {
    s.push_str(" world!");
    let slen = s.len();
    (s, slen)
}