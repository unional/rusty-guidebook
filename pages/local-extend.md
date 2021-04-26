# Local Extend

Local extend is a way to use `impl` to organize your code.

It works like OO inheritence, but not creating a child class.

```rs
// dog.rs
struct Dog {}

impl Dog {
  fn bark(&self) {
    "woof"
  }
}

// pet_store.rs
mod dog;

impl Dog {
  fn meow(&self) {
    "meow"
  }
}
```

In `pet_store`, we teach our `Dog` a new trick.
