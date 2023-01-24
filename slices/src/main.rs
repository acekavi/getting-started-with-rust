fn main() {
    let text = String::from("quick brown fox jump up the box");

    println!("{}",first_word(&text));
}

fn first_word(input_text: &String) -> &str{
    let bytes = input_text.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input_text[0..i];
        }
    }

    &input_text[..]
}
