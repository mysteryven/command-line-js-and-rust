use clap::{App, Arg};
use std::{
    error::Error,
    fmt::format,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("mysterven mysteryven@gmail.com")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("INPUT_FILE")
                .default_value("-")
                .index(1)
                .help("input file"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUTPUT_FILE")
                .index(2)
                .help("out file"),
        )
        .arg(
            Arg::with_name("count")
                .takes_value(false)
                .help(
                    " Precede each output line with the count of the number of times the
    line occurred in the input",
                )
                .long("count")
                .short("c"),
        )
        .get_matches();

    let in_file = matches.value_of("in_file").map(String::from).unwrap();
    let out_file = matches.value_of("out_file").map(String::from);

    Ok(Config {
        in_file,
        out_file,
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{} {}", config.in_file, e))?;
    let mut line = String::new();
    let mut last: String = String::new();
    let mut last_count: usize = 0;
    let mut index = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            print_content(last_count, config.count, &last, &config.out_file);
            break;
        }

        if last != line && index != 0 {
            print_content(last_count, config.count, &last, &config.out_file);
            last_count = 1;
        } else {
          last_count += 1
        }

        last = line.clone();

        line.clear();
        index += 1;
    }

    Ok(())
}

pub fn format_count(count: usize, show: bool) -> String {
    if show == true {
        format!("{:>4} ", count)
    } else {
        "".to_string()
    }
}

pub fn print_content(count: usize, show_count: bool, content: &str, out_file: &Option<String>) {
    match out_file {
        Some(filename) => {
            print!("{}{}", format_count(count, show_count), content);
        }
        None => {
            print!("{}{}", format_count(count, show_count), content);
        }
    }
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
