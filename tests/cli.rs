use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn prints_valid_lru() -> TestResult {
    let mut cmd = Command::cargo_bin("memory-replacement")?;
    cmd.args(&["1 2 3 4 1 2 5 1 2 3 4 5", "-f", "3", "-t", "lru"]);
    let out = cmd.assert().success();
    insta::assert_snapshot!(str::from_utf8(&out.get_output().stdout)?);

    Ok(())
}


#[test]
fn prints_valid_fifo() -> TestResult {

    // 3 frames
    let mut cmd = Command::cargo_bin("memory-replacement")?;
    cmd.args(&["1 2 3 4 1 2 5 1 2 3 4 5", "-f", "3", "-t", "fifo"]);
    let out = cmd.assert().success();
    insta::assert_snapshot!(str::from_utf8(&out.get_output().stdout)?);

    // 4 frames
    let mut cmd = Command::cargo_bin("memory-replacement")?;
    cmd.args(&["1 2 3 4 1 2 5 1 2 3 4 5", "-f", "4", "-t", "fifo"]);
    let out = cmd.assert().success();
    insta::assert_snapshot!(str::from_utf8(&out.get_output().stdout)?);
    
    Ok(())
}

#[test]
fn prints_valid_opt() -> TestResult {
    // 3 frames
    let mut cmd = Command::cargo_bin("memory-replacement")?;
    cmd.args(&["1 2 3 4 1 2 5 1 2 3 4 5", "-f", "3", "-t", "opt"]);
    let out = cmd.assert().success();
    insta::assert_snapshot!(str::from_utf8(&out.get_output().stdout)?);

    Ok(())
}

#[test]
fn prints_valid_opt_2() -> TestResult {
    // 4 frames
    let mut cmd = Command::cargo_bin("memory-replacement")?;
    cmd.args(&["1 2 3 4 1 2 5 1 2 3 4 5", "-f", "4", "-t", "opt"]);
    let out = cmd.assert().success();
    insta::assert_snapshot!(str::from_utf8(&out.get_output().stdout)?);

    Ok(())
}