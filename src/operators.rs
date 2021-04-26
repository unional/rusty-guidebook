#[test]
fn plus_operator() {
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = "".to_string() + &s1 + &s2; // note s1 has been moved here and can no longer be used

  println!("s1: {}", s1);
  println!("s3: {}", s3);
}
