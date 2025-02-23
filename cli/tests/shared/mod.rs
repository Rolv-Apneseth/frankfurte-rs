#![allow(dead_code)]

use assert_cmd::Command;

pub const BIN: &str = "frs";

pub fn get_cmd() -> Command {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("--url=http://localhost:8080");
    cmd
}
