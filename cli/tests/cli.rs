use assert_cmd::Command;
use predicates::{
    prelude::PredicateBooleanExt,
    str::{contains, ends_with, is_match, starts_with},
};

fn get_cmd() -> Command {
    let mut cmd = Command::cargo_bin("frs").unwrap();
    cmd.arg("--url=http://localhost:8080");
    cmd
}

#[test]
fn test_currencies_basic() {
    get_cmd()
        .arg("currencies")
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
fn test_currencies_json() {
    get_cmd()
        .arg("currencies")
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
fn test_currencies_raw() {
    get_cmd()
        .arg("currencies")
        .arg("--raw")
        .assert()
        .stdout(starts_with("AUD\tAustralian Dollar").and(contains("EUR\tEuro")))
        .success();
}

#[test]
fn test_convert_basic() {
    get_cmd()
        .arg("convert")
        .assert()
        .stdout(
            contains("AUD")
                .and(contains("USD"))
                .and(contains("GBP"))
                .and(is_match("\\d").unwrap()),
        )
        .success();
}

#[test]
fn test_convert_targets() {
    get_cmd()
        .args(["convert", "USD", "EUR,GBP"])
        .assert()
        .stdout(
            contains("EUR")
                .and(contains("GBP"))
                .and(is_match("\\d").unwrap())
                .and(contains("USD").not()),
        )
        .success();
}

#[test]
fn test_convert_amount() {
    get_cmd()
        .args(["convert", "-a", "1000", "--json"])
        .assert()
        .stdout(contains("1000").and(is_match("\\d").unwrap()))
        .success();
}

#[test]
fn test_period_basic() {
    get_cmd()
        .args(["period", "EUR", "2024-10-10"])
        .assert()
        .stdout(
            contains("2024-10-10")
                .and(contains("2024-10-11"))
                .and(contains("2024-11-05"))
                .and(contains("AUD"))
                .and(contains("USD"))
                .and(contains("GBP")),
        )
        .success();
}

#[test]
fn test_period_end_date() {
    get_cmd()
        .args(["period", "EUR", "2020-5-12", "2020-5-13"])
        .assert()
        .stdout(
            contains("2020-05-12")
                .and(contains("2020-05-13"))
                .and(contains("2024-05-11").not())
                .and(contains("2024-05-14").not()),
        )
        .success();
}
