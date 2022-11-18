use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: usize,
}

pub fn get_args() {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("mysteryven")
        .about("Rust head")
        .arg(
            Arg::with_name("line")
                .short("n")
                .long("line")
                .takes_value(true)
                .default_value("10")
                .conflicts_with("bytes")
                .help("Print count lines of each of the specified files."),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .takes_value(true)
                .help("Print bytes of each of the specified files."),
        )
        .arg(Arg::with_name("files").multiple(true).default_value("-"))
        .get_matches();

    let lines = matches.value_of("line").try_into();
    let bytes = matches.value_of("bytes");
    let files = matches.values_of_lossy("files").unwrap();

    println!("{:?}, {:?}", lines, bytes)

    Ok(Config {
      lines,
      bytes,
      files
    })
}
