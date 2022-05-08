fn main() {
  let s1 = gives_ownership();
  let s2 = String::from("hello");
  let s3 = takes_and_gives_back(s2);
// let (s4, len) = calculate_length(s3);
// println!("{}, {}", s4, len);
  let len = calculate_length_reference(&s3);
  println!("{}, {}", s3, len);
  println!("{}, {}", s3, len);

  mutate_string();
}

fn gives_ownership() -> String {
  let some_string = String::from("hello");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}


fn calculate_length_reference(s: &String) -> usize {
    let length = s.len();
    length
}

fn mutate_string() {
    let mut s = String::from("hello 1");
    let _r1 = &mut s;
    // let _r2 = &mut s;
    // let _r3 = &s;
    println!("{}", _r1);
}
  