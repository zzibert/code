use std::convert::TryInto; // Enables to be called on those types that have implemented it (suck as u16)

fn main() {
  let a: i32 = 10;
  let b: u16 = 100;

  let b_ = b.try_into().unwrap();

  if a < b_ {
    println!("Ten is less than one hundred.")
  }
}

