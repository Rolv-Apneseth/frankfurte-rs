use chrono::NaiveDate;
use reqwest::StatusCode;

use crate::data::Currency;

/// [`std::result::Result`] wrapper for convenience.
pub(super) type Result<T> = std::result::Result<T, Error>;

/// Possible errors that can be encountered when making requests using the [`crate::api::ServerClient`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The target currencies '{}' include the base currency '{base}'", .targets.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(","))]
    RequestTargetsIncludeBase {
        base: Currency,
        targets: Vec<Currency>,
    },

    #[error("Provided end date ({end}) is before the start date ({start})")]
    RequestEndDateBeforeStart { start: NaiveDate, end: NaiveDate },

    #[error("The start date ({0}) is past the latest date with exchange rates")]
    RequestLateStartDate(NaiveDate),

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
