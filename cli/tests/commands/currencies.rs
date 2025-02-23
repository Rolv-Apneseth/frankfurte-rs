use crate::shared::get_cmd;
use predicates::{
    prelude::PredicateBooleanExt,
    str::{contains, ends_with, is_match, starts_with},
};

pub(super) const COMMAND: &str = "currencies";

// SUCCESS CASES -----------------------------------------------------------------------------------
#[test]
fn test_basic() {
    get_cmd()
        .arg(COMMAND)
        .assert()
        .stdout(
            contains("AUD")
                .and(contains("USD"))
                .and(contains("GBP"))
                .and(contains("United States Dollar"))
                .and(is_match("\\d").unwrap().not()),
        )
        .success();
}

#[test]
fn test_json() {
    get_cmd()
        .arg(COMMAND)
        .arg("--json")
        .assert()
        .stdout(
            starts_with("{")
                .and(ends_with("}\n"))
                .and(contains("\"EUR\": \"Euro\"")),
        )
        .success();
}

#[test]
fn test_raw() {
    get_cmd()
        .arg(COMMAND)
        .arg("--raw")
        .assert()
        .stdout(starts_with("AUD\tAustralian Dollar").and(contains("EUR\tEuro")))
        .success();
}
