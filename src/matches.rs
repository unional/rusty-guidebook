pub fn match_range() {
  let x = 5;

  // `1..=5` is inclusive range
  // `1..5` means 1 | 2 | 3 | 4, which is typically not what you want
  match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
  }
}
