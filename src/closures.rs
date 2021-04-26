#[cfg(test)]
mod curry {
  fn curry_plus(a: u8) -> Box<dyn Fn(u8) -> u8> {
    Box::new(move |b| a + b)
  }

  #[test]
  fn plus_curry() {
    let p = curry_plus(8);
    assert_eq!(16, p(8));
  }

  #[test]
  fn mutate_captured() {
    let mut s = "hello";

    // closure does a mutable borrow of s
    let mut changer = || s = "world";

    changer();
    // does an immutable borrow of s
    assert_eq!(s, "world");
  }
}
