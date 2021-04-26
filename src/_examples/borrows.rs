#[cfg(test)]
mod tests {
  #[test]
  fn multiple_immut_borrows() {
    let s = String::from("Hello, World");

    let b1 = &s;
    let b2 = &s;
    let b3 = &s;

    println!("b1: {}, b2: {}, b3, {}", b1, b2, b3);
  }

  #[test]
  fn cannot_mul_borrow_with_immut_borrower() {
    let mut s = String::from("Hello, World");

    // compile error
    // let b1 = &s;

    s.push_str("world");

    // compile error
    // println!("b1: {}", b1);
  }
}
