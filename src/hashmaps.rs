#[test]
fn main() {
  use std::collections::HashMap;

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  // field_name and field_value are invalid at this point, try using them and
  // see what compiler error you get!
}

#[test]
fn main2() {
  use std::collections::HashMap;

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    if let Some(v) = map.insert(word, 0) {
      println!("v: {:?}", v);
    }
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}
