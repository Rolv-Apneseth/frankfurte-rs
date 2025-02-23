mod shared;

use assert_cmd::Command;
use predicates::{prelude::PredicateBooleanExt, str::contains};
use shared::BIN;

#[test]
fn test_invalid_api() {
    const URL: &str = "http://localhost";
    Command::cargo_bin(BIN)
        .unwrap()
        .arg(format!("--url={URL}"))
        .arg("currencies")
        .assert()
        .stderr(contains("error sending request"))
        .failure();
}

#[test]
fn test_invalid_api_endpoint() {
    const URL: &str = "http://localhost:8080/invalid";
    Command::cargo_bin(BIN)
        .unwrap()
        .arg(format!("--url={URL}"))
        .arg("currencies")
        .assert()
        .stderr(
            contains("URL")
                .and(contains(URL))
                .and(contains("Status"))
                .and(contains("404")),
        )
        .failure();
}
