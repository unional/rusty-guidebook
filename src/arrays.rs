#[test]
fn collect_vector() {
  let s = String::from("hello world");
  let words: Vec<&str> = s.split_whitespace().collect();
  println!("Words {:?}", words);
}
