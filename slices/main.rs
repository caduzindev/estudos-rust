fn main() {
  let teste = String::from("Carlos Eduardo");
  let array = [1,2,3,4];
  let teste2 = fatiador_array(&array);

  println!("{}", first_word(&teste));
  println!("{}", teste2.len());
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

fn fatiador_array(n: &[i32]) -> &[i32] {
  &n[0..3]
}