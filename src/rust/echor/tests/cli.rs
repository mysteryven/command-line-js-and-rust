use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn die_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

fn runs(args: &[&str], expect: &str) -> TestResult {
    let expected = fs::read_to_string(expect)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    runs(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello1n() -> TestResult {
    runs(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2() -> TestResult {
    runs(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello2n() -> TestResult {
    runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
