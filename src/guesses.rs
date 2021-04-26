pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: String) -> Guess {
    let value: i32 = value.trim().parse().unwrap();

    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }

    Guess { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

#[test]
pub fn get_guess() {
  println!("Guess the number!");

  println!("Please input your guess.");

  let guess = String::from("42");

  let guess = Guess::new(guess).value();
  assert_eq!(42, guess);
}
