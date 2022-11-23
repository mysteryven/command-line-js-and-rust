use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_bytes: usize,
    num_chars: usize,
    num_words: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("mysteryven mysteryven@gmail.com")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .required(true)
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .takes_value(false)
                .help("Show line count")
                .short("l")
                .long("lines"),
        )
        .arg(
            Arg::with_name("words")
                .takes_value(false)
                .help("Show word count")
                .short("w")
                .long("words"),
        )
        .arg(
            Arg::with_name("bytes")
                .takes_value(false)
                .short("c")
                .help("Show byte count")
                .long("bytes"),
        )
        .arg(
            Arg::with_name("chars")
                .takes_value(false)
                .help("Show character count")
                .conflicts_with("bytes")
                .short("m")
                .long("chars"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut bytes = matches.is_present("bytes");
    let mut words = matches.is_present("words");
    let chars = matches.is_present("chars");

    if [lines, bytes, chars, words].iter().all(|v| !v) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        bytes,
        chars,
        words,
    })
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", &filename, err),
            Ok(buf_reader) => {
                if let Ok(info) = count(buf_reader) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_bytes, config.chars),
                        if filename.as_str() == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }

    Ok(())
}

pub fn format_field(name: usize, show: bool) -> String {
    if show {
        format!("{:>8}", name)
    } else {
        "".to_string()
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut num_words = 0;

    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        num_lines += 1;
        num_bytes += bytes;
        num_chars += line.chars().count();
        num_words += line.split_whitespace().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_bytes,
        num_chars,
        num_words,
    })
}

#[cfg(test)]
mod test {
    use crate::format_field;

    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected)
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, true), "       1");
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(10, true), "      10")
    }
}
