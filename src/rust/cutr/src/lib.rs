use crate::Extract::*;
use clap::{App, Arg};
use regex::Regex;
use std::{
    error::Error,
    num::{NonZeroIsize, NonZeroUsize},
    ops::Range,
};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

pub fn get_args() -> MyResult<()> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("mysteryven <mysteryven@gmail.com>")
        .about("Rust cut")
        // What goes here?
        .get_matches();

    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", &config);
    Ok(())
}

fn parse_index(input: &str) -> Result<usize, String> {
    let value_error = || format!("illegal list value: \"{}\"", input);

    input
        .starts_with("+")
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
                .map_err(|_| value_error())
        })
}

fn parse_to_int(val: &str, input: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(v) => {
            if v > 0 {
                Ok(v)
            } else {
                Err(From::from(format!("illegal list value: \"{}\"", v)))
            }
        }
        _ => Err(From::from(format!("illegal list value: \"{}\"", input))),
    }
}

#[cfg(test)]
mod unit_tests {
    use super::parse_pos;
    #[test]
    fn test_parse_pos() {
        // The empty string is an error assert!(parse_pos("").is_err());
        // Zero is an error

        // All the following are acceptable
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);
        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);
        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);
        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);
        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
