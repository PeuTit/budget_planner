use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn no_flag_supplied() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("budget_planner")?;

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided:",
    ));

    Ok(())
}

#[test]
fn wrong_flag_supplied() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("budget_planner")?;

    cmd.arg("--yaer");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));

    Ok(())
}

#[test]
fn no_argument_supplied() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("budget_planner")?;

    cmd.arg("--year");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("a value is required"));

    Ok(())
}

#[test]
fn wrong_argument_supplied() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("budget_planner")?;

    cmd.arg("--year").arg("asdf");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));

    Ok(())
}

#[test]
fn correct_argument_supplied() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("budget_planner")?;

    cmd.arg("--year").arg("2023");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2023"));

    Ok(())
}
