use lib_frankfurter::{CurrencyValue, ValidDate};
use predicates::{
    prelude::PredicateBooleanExt,
    str::{contains, is_match},
};

use crate::shared::get_cmd;

use super::INVALID_DATES;

pub(super) const COMMAND: &str = "convert";

// SUCCESS CASES -----------------------------------------------------------------------------------
#[test]
fn test_basic() {
    get_cmd()
        .args([COMMAND, "EUR"])
        .assert()
        .stdout(
            contains("AUD")
                .and(contains("USD"))
                .and(contains("GBP"))
                .and(contains("EUR").not())
                .and(is_match("\\d").unwrap()),
        )
        .success();
}

#[test]
fn test_no_base() {
    get_cmd()
        .arg(COMMAND)
        .assert()
        .stdout(
            contains("NOK")
                .and(contains("PHP"))
                .and(contains("EUR").not()),
        )
        .success();
}

#[test]
fn test_targets() {
    get_cmd()
        .args([COMMAND, "USD", "EUR,GBP"])
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
fn test_date() {
    let output_latest =
        String::from_utf8(get_cmd().args([COMMAND]).output().unwrap().stdout).unwrap();

    for date in [*ValidDate::min(), *ValidDate::max()] {
        let date = format!("{}", date.format("%Y-%m-%d"));
        get_cmd()
            .args([COMMAND, "--date", &date])
            .assert()
            .success();
    }

    for date in ["1999-12-29", "2023-03-04", "2024-02-02"] {
        let output_date = String::from_utf8(
            get_cmd()
                .args([COMMAND, "--date", date])
                .assert()
                .success()
                .get_output()
                .stdout
                .clone(),
        )
        .unwrap();
        assert_ne!(output_latest, output_date);
    }
}

#[test]
fn test_amount_basic() {
    // Values which can be checked directly (no rounding or displaying in scientific notation, etc.)
    for amount in [
        (*CurrencyValue::MIN).to_string().as_str(),
        (*CurrencyValue::MAX).to_string().as_str(),
        "1",
        "2",
        "5",
        "10",
        "100",
        "1000000",
    ] {
        get_cmd()
            .args([COMMAND, "-a", amount, "--json"])
            .assert()
            .stdout(contains(amount).and(contains("PHP")))
            .success();
    }

    // Other values
    for amount in ["0.012345", "100.481983", "7821.234"] {
        get_cmd()
            .args([COMMAND, "-a", amount, "--json"])
            .assert()
            .stdout(is_match("\\d").unwrap().and(contains("PHP")))
            .success();
    }
}

#[test]
fn test_amount_ignore_separators() {
    const AMOUNT: &str = "1000";
    for amount in ["1_000_000", "1,000,000", "1,0,0,__0,0_0,0"] {
        get_cmd()
            .args([COMMAND, "-a", amount, "--json"])
            .assert()
            .stdout(contains(AMOUNT).and(contains("USD")))
            .success();
    }
}

// FAILURE CASES -----------------------------------------------------------------------------------
#[test]
fn test_fail_invalid_date() {
    for date in INVALID_DATES {
        get_cmd().args([COMMAND, "-d", date]).assert().failure();
    }
}
