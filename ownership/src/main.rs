fn main(){
  let s1  = String::from("Hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of {s2} is {len}");
}

fn calculate_length(text: String) -> (String, usize){
  let length = text.len();
  (text, length)
}