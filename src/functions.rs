#[cfg(test)]
mod basic {
  fn add(a: u8, b: u8) -> u8 {
    a + b
  }

  #[test]
  fn primitive_is_copied() {
    let x = 1;
    let y = 2;
    let actual = add(x, y);

    assert_eq!(1, x);
    assert_eq!(3, actual);
  }

  struct Blackboard {}

  trait Draw {
    fn draw(&self);
  }

  impl Draw for Blackboard {
    fn draw(&self) {
      println!("drawing with chalk");
    }
  }

  fn consume_foo(foo: impl Draw) {
    foo.draw()
  }

  #[test]
  fn using_trait_in_fn_param() {
    let b = Blackboard {};
    consume_foo(b);
  }
}
