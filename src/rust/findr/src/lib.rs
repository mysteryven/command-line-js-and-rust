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
            Arg::with_name("name")
                .value_name("NAME")
                .short("n")
                .long("name")
                .takes_value(true)
                .multiple(true)
                .help("Name"),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .short("t")
                .multiple(true)
                .possible_values(&["f", "d", "l"])
                .long("type"),
        )
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .default_value(".")
                .multiple(true),
        )
        .get_matches();

    let paths = matches.values_of_lossy("path").unwrap();
    let mut names_reg: Vec<Regex> = vec![];
    let mut entry_types_final: Vec<EntryType> = vec![];

    if matches.is_present("name") {
        let names = matches.values_of_lossy("name").unwrap();
        for name in names {
            names_reg.push(Regex::new(&name)?)
        }
    }

    if matches.is_present("type") {
        let entry_types = matches.values_of_lossy("type").unwrap();
        for entry_type in entry_types {
            if &entry_type == "f" {
                entry_types_final.push(File);
            } else if &entry_type == "l" {
                entry_types_final.push(Link);
            } else {
                entry_types_final.push(Dir);
            }
        }
    }

    Ok(Config {
        paths,
        names: names_reg,
        entry_types: entry_types_final,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);

    Ok(())
}
