use chrono::NaiveDate;

use crate::data::Currency;

pub type Result<T> = std::result::Result<T, Error>;

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
