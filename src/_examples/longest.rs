pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_longest_between_two_scopes() {
    let string1 = String::from("long string is long");

    {
      let string2 = String::from("xyz");
      let result = longest(string1.as_str(), string2.as_str());
      println!("The longest string is {}", result);
      println!("1: {} 2:{}", string1, string2);
    }
  }
}
