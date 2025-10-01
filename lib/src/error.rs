use reqwest::StatusCode;

use crate::{data::Currency, CurrencyValue, ValidDate};

/// [`std::result::Result`] wrapper for convenience.
pub(super) type Result<T> = std::result::Result<T, Error>;

/// Possible errors that can be encountered when making requests using the [`crate::api::ServerClient`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The target currencies '{}' include the base currency '{base}'", .targets.iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(","))]
    RequestTargetsIncludeBase {
        base: Currency,
        targets: Vec<Currency>,
    },

    #[error("Provided end date ({end}) is before the start date ({start})")]
    RequestEndDateBeforeStart { start: ValidDate, end: ValidDate },

    #[error(
        "Invalid currency value ({0}), must be a valid number between {min} and {max}",
        min = *CurrencyValue::MIN,
        max = *CurrencyValue::MAX
    )]
    InvalidCurrencyValue(String),

    #[error(
        "Invalid date provided ({0}), must be a valid date in the form yyyy-mm-dd between {min} and {max}",
        min = ValidDate::min().format("%Y-%m-%d"),
        max = ValidDate::max().format("%Y-%m-%d"),
    )]
    InvalidDate(String),

    #[error("Invalid response from the API\n  URL - {url}\n  Status - {status}\n  Body - {body}")]
    InvalidResponse {
        url: String,
        status: StatusCode,
        body: String,
    },

    /// Error from [`reqwest`], see [`reqwest::Error`].
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// Error from [`serde_json`], see [`serde_json::Error`].
    #[error(transparent)]
    SerdeJSON(#[from] serde_json::Error),
    /// Error from [`std::io`], see [`std::io::Error`].
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
