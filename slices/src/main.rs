fn main() {
    let text = String::from("Quick brown fox jumps over the wall");

    println!("{}",first_word(&text));
}

fn first_word(input_text: &String) -> &str{
    let bytes = input_text.as_bytes();

    let count:u8 = 4;
    let mut start_index:usize = 0;
    let mut num_of_spaces:u8 = 0;

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            if num_of_spaces == (count -1) {
                return &input_text[start_index..i];
            }else{
                start_index = i;
            }
            num_of_spaces += 1;
        }
    }

    &input_text[..]
}
