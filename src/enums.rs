#[cfg(example)]
mod basic {
  #[derive(Debug)]
  enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
  }

  fn dump(v: &Value) {
    use Value::*;
    match *v {
      Number(n) => println!("number is {}", n),
      Str(ref s) => println!("string is '{}'", s),
      Bool(b) => println!("boolean is {}", b),
    }
  }

  impl Value {
    fn as_str(v: &Value) -> Option<&String> {
      use Value::*;
      match *v {
        Str(ref s) => Some(s),
        _ => None,
      }
    }
    pub fn to_str(self) -> Option<String> {
      match self {
        Value::Str(s) => Some(s),
        _ => None,
      }
    }
  }
}
