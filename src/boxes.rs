#[test]
fn assume_ownership() {
  fn foo() -> Box<&'static str> {
    Box::new("hello world")
  }

  let actual = foo();

  println!("greet: {}", actual);
  assert_eq!("hello world", *actual);
}
