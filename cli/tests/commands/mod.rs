mod convert;
mod currencies;
mod period;

use crate::shared::get_cmd;

pub(super) const INVALID_DATES: [&str; 11] = [
    "date",
    "1st of January, 2000",
    "1999/12/29",
    "20240201",
    "0000-01-01",
    "0001-10-10",
    "1900-10-10",
    "1000-09-22",
    "3005-01-01",
    "2024-35-01",
    "2024-01-70",
];

// SHARED FAILURE CASES ----------------------------------------------------------------------------
#[test]
fn test_fail_raw_and_json() {
    for cmd in [convert::COMMAND, currencies::COMMAND, period::COMMAND] {
        get_cmd().args([cmd, "--raw", "--json"]).assert().failure();
    }
}

#[test]
fn test_fail_targets_before_base() {
    for cmd in [convert::COMMAND, period::COMMAND] {
        get_cmd().args([cmd, "EUR,GBP"]).assert().failure();
    }
}

#[test]
fn test_fail_invalid_amount() {
    for cmd in [convert::COMMAND, period::COMMAND] {
        for amount in ["0", "0.0", "0.009", "0.000001"] {
            get_cmd().args([cmd, "-a", amount]).assert().failure();
        }
    }
}
