trait Barker {
  fn bark(&self);
}

trait Barker2 {
  fn bark(&self);
}

struct Human {}

impl Barker for Human {
  fn bark(&self) {}
}

impl Barker2 for Human {
  fn bark(&self) {
    println!("Bark: ...");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn barking() {
    let you = Human {};
    Barker::bark(&you);
  }
}
