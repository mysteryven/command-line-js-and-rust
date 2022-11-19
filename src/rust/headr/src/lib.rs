use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
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
                .long("bytes")
                .takes_value(true)
                .help("Print bytes of each of the specified files."),
        )
        .arg(Arg::with_name("files").multiple(true).default_value("-"))
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = parse_positive_int(matches.value_of("line").unwrap()).unwrap();
    let bytes = match matches.value_of("bytes") {
        Some(v) => Some(parse_positive_int(v).unwrap()),
        _ => None,
    };

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
