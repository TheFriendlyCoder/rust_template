use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn hello_world() {
    let value = true;
    assert!(value == true);
}

#[test]
#[should_panic]
fn test_panic_condition() {
    panic!("Darn it");
}

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin("sample")?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}
