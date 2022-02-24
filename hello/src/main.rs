fn greet_world() {
  println!("Hello, world!");
  let german = "Gruss gott!";
  let slovene = "trolololo";
  let regions = [german, slovene];

  for region in regions.iter() {
      println!("{}", &region);
  }
}

fn main() {
  greet_world();
}
