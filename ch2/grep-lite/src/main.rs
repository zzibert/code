use regex::Regex;    // <1>
use clap::{App,Arg};

fn main() {
  let args = App::new("grep-lite")
    .version("0.1")
    .about("searches for patterns")
    .arg(Arg::with_name("pattern")
      .help("the pattern to search for")
      .takes_value(true)
      .required(true))
    .get_matches();

  let pattern = args.value_of("pattern").unwrap();
  let re = Regex::new(pattern).unwrap();

  let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

  for line in quote.lines() {
    let contains_substring = re.find(line);
    match contains_substring {    // <3>

        Some(_) => println!("{}", line),    // <4>
        None => (),    // <5>
    }
  }
}

