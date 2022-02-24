fn main() {
  let twenty = 20; // rust infers a type on your behalf if you dont supply one
  let twenty_one: i32 = 21; // which is done by adding type annotations
  let twenty_two = 22i32; // or type suffix

  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  let one_million: i64 = 1_000_000; // underscores are ignored by compiler
  println!("{}", one_million.pow(2)); // numbers have methods

  let forty_twos = [
    42.0,
    42f32,
    42.0_f32,
  ];
  println!("{:02}", forty_twos[0]);
}