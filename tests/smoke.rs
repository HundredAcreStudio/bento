#![allow(clippy::expect_used, clippy::unwrap_used)]

use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::{contains, starts_with};

#[test]
fn prints_version() {
    let mut cmd = Command::cargo_bin("bento").expect("binary should be built by cargo test");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(starts_with("bento "));
}

#[test]
fn repl_echoes_until_eof() {
    let mut cmd = Command::cargo_bin("bento").expect("binary should be built by cargo test");
    cmd.write_stdin("hello\nworld\n")
        .assert()
        .success()
        .stdout(contains("hello").and(contains("world")));
}

#[test]
fn repl_exits_cleanly_on_immediate_eof() {
    let mut cmd = Command::cargo_bin("bento").expect("binary should be built by cargo test");
    cmd.write_stdin("").assert().success();
}
