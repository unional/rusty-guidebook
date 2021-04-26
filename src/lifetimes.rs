#[derive(Debug)]
pub struct A {
  s: String,
}

impl A {
  pub fn new() -> A {
    A {
      s: String::from("I'm a little string"),
    }
  }
}

#[test]
fn lifetime() {
  let a = A::new();
  println!("a: {:?}", a);
}
