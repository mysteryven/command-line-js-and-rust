use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
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
    let mut file = open(&config.in_file).map_err(|e| format!("{}:  {}", config.in_file, e))?;
    let mut line = String::new();
    let mut previous: String = String::new();
    let mut count: usize = 0;

    let mut write = get_writer(&config.out_file)?;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            if count > 0 {
                print_content(count, config.count, &previous, &mut write)?;
            }
            break;
        }

        if line.trim_end() != previous.trim_end() {
            if count > 0 {
                print_content(count, config.count, &previous, &mut write)?;
                count = 0;
            }
            previous = line.clone();
        }

        count += 1;
        line.clear();
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

pub fn print_content(
    count: usize,
    show_count: bool,
    content: &str,
    writer: &mut Box<dyn Write>,
) -> MyResult<()> {
    let content = format!("{}{}", format_count(count, show_count), content);
    writer.write(content.as_bytes())?;
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_writer(out_file: &Option<String>) -> MyResult<Box<dyn Write>> {
    match out_file {
        Some(filename) => Ok(Box::new(BufWriter::new(File::create(filename)?))),
        None => Ok(Box::new(BufWriter::new(io::stdout()))),
    }
}
