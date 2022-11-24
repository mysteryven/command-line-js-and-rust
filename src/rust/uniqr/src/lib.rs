use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
  in_file: String,
  out_file: Option<String>,
  count: bool
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {

  let matches = App::new("uniqr").version("0.1.0")
  .author("mysterven mysteryven@gmail.com")
  .about("Rust uniq")
  .arg(
    Arg::with_name("in_file").value_name("INPUT_FILE").default_value("-").index(1).help("input file")
  ).arg(
    Arg::with_name("out_file").value_name("OUTPUT_FILE").index(2).help("out file")
  ).arg(
    Arg::with_name("count").takes_value(false).help(" Precede each output line with the count of the number of times the
    line occurred in the input").long("count").short("c")
  ).get_matches();

  
  let in_file = matches.value_of("in_file").map(String::from).unwrap();
  let out_file = matches.value_of("out_file").map(String::from);

  Ok(Config {
    in_file,
    out_file,
    count: matches.is_present("count")
  })
}

pub fn run(config: Config) -> MyResult<()> {

  println!("{:?}", config);

  Ok(())
}