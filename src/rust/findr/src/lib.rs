use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("mysteryven <mysteryven@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .takes_value(true)
                .multiple(true)
                .help("Name"),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .multiple(true)
                .possible_values(&["f", "d", "l"])
                .long("type"),
        )
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .default_value(".")
                .multiple(true),
        )
        .get_matches();

    let paths = matches.values_of_lossy("path").unwrap();

    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches.values_of_lossy("types").map(|vals| {
        vals.into_iter().map(|val| match val.as_str() {
            "f" => File,
            "d" => Dir,
            "l" => Link,
            _ => unreachable!("Invalid type")
        }).collect()
    }).unwrap_or_default();

    Ok(Config {
        paths,
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);

    Ok(())
}
