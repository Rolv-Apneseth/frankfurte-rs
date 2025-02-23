use crate::shared::get_cmd;
use chrono::Days;
use lib_frankfurter::ValidDate;
use predicates::{prelude::PredicateBooleanExt, str::contains};
use std::str::FromStr;

use super::INVALID_DATES;

pub(super) const COMMAND: &str = "period";

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
                .and(contains("EUR").not()),
        )
        .success();
}

#[test]
fn test_no_base() {
    get_cmd()
        .arg(COMMAND)
        .assert()
        .stdout(contains("AUD").and(contains("USD")).and(contains("GBP")))
        .success();
}

#[test]
fn test_start_date() {
    for date in [*ValidDate::min(), *ValidDate::max()] {
        let date = format!("{}", date.format("%Y-%m-%d"));
        get_cmd()
            .args([COMMAND, "--start", &date])
            .assert()
            .success();
    }

    for date in ["1999-12-29", "2023-03-04", "2024-02-02"] {
        get_cmd()
            .args([COMMAND, "EUR", "--start", date])
            .assert()
            .stdout(
                contains("2024-10-10")
                    .and(contains("2024-10-11"))
                    .and(contains("2024-11-05"))
                    .and(contains("2025-01-06"))
                    .and(contains("AUD"))
                    .and(contains("USD"))
                    .and(contains("GBP")),
            )
            .success();
    }
}

#[test]
fn test_end_date() {
    for date in ["2022-06-14", "2023-03-06", "2024-02-01"] {
        let next_date = (*ValidDate::from_str(date).unwrap())
            .checked_add_days(Days::new(1))
            .unwrap();
        let next_date = format!("{}", next_date.format("%Y-%m-%d"));

        get_cmd()
            .args([COMMAND, "EUR", "-s", "2022-05-12", "-e", date])
            .assert()
            .stdout(
                contains("2022-06-14")
                    .and(contains(date))
                    .and(contains("2024-02-02").not())
                    .and(contains(&next_date).not()),
            )
            .success();
    }
}

// FAILURE CASES -----------------------------------------------------------------------------------
#[test]
fn test_fail_invalid_start_date() {
    for date in INVALID_DATES {
        get_cmd().args([COMMAND, "-s", date]).assert().failure();
    }
}

#[test]
fn test_fail_invalid_end_date() {
    for date in ["3005-01-01", "2024-01-01", "2025-01-31", "0000-01-01"] {
        get_cmd()
            .args([COMMAND, "-s", "2025-02-01", "-e", date])
            .assert()
            .failure();
    }
}
