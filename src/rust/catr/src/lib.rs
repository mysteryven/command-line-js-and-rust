use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_line: bool,
    number_nonblank_line: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("mysteryven <mysteryven@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("number_line")
                .short("n")
                .long("number")
                .takes_value(false)
                .conflicts_with("number_nonblank_line")
                .help("Number the output lines, starting at 1."),
        )
        .arg(
            Arg::with_name("number_nonblank_line")
                .short("b")
                .long("number-nonblank")
                .takes_value(false)
                .help("Number the non-blank output lines, starting at 1"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .multiple(true)
                .required(true)
                .help("Input file(s)")
                .default_value("-"),
        )
        .get_matches();

    let number_line = matches.is_present("number_line");
    let number_nonblank_line = matches.is_present("number_nonblank_line");
    let files = matches.values_of_lossy("files").unwrap();

    Ok(Config {
        files,
        number_line,
        number_nonblank_line,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        println!("{}", filename)
    }

    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "_" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
