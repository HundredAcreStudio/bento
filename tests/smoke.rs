#![allow(clippy::expect_used, clippy::unwrap_used)]

use assert_cmd::Command;
use predicates::str::starts_with;

#[test]
fn prints_version() {
    let mut cmd = Command::cargo_bin("bento").expect("binary should be built by cargo test");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(starts_with("bento "));
}
