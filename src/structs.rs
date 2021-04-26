pub struct Person {
  pub first_name: String,
  pub last_name: String,
}

impl Person {
  pub fn new(first: &str, name: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: name.to_string(),
    }
  }
  pub fn to_tuple(&self) -> (&str, &str) {
    (&self.first_name, &self.last_name)
  }
}

#[test]
fn try_to_tuple() {
  let p = Person::new("John", "doe");
  p.to_tuple();
}
