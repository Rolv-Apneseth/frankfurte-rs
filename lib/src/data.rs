use std::{collections::BTreeMap, fmt::Display, num::ParseFloatError, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantNames};

/// A map of [`Currency`] to their respective [`CurrencyValue`], sorted by the currency code keys.
///
/// This represents a JSON response from the server outlining exchange rates for different currency
/// ISO 4217 codes.
pub type CurrencyValueMap = BTreeMap<Currency, CurrencyValue>;

/// Possible currency codes (ISO 4217) returned by the `Frankfurter` API.
#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Deserialize,
    Serialize,
    EnumString,
    VariantNames,
    Display,
)]
#[strum(ascii_case_insensitive)]
#[allow(clippy::upper_case_acronyms)]
#[non_exhaustive]
pub enum Currency {
    AUD,
    BGN,
    BRL,
    CAD,
    CHF,
    CNY,
    CZK,
    DKK,
    EUR,
    GBP,
    HKD,
    HUF,
    IDR,
    ILS,
    INR,
    ISK,
    JPY,
    KRW,
    MXN,
    MYR,
    NOK,
    NZD,
    PHP,
    PLN,
    RON,
    SEK,
    SGD,
    THB,
    TRY,
    USD,
    ZAR,

    /// Support for other currency codes than the ones listed above.
    ///
    /// This is necessary when fetching rates for older dates, as there are rates available for
    /// different currencies.
    #[serde(untagged)]
    #[strum(serialize = "{0}")]
    Other(String),
}

impl Default for Currency {
    fn default() -> Self {
        Self::EUR
    }
}

/// Value of a currency. Simple wrapper around an [`f64`].
///
/// This is a wrapper around [`f64`] to ensure that values are rounded to 2 decimal places.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct CurrencyValue(f64);

impl Display for CurrencyValue {
    // Limit to 2 decimal places
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl From<f64> for CurrencyValue {
    fn from(value: f64) -> Self {
        CurrencyValue(value)
    }
}

impl FromStr for CurrencyValue {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CurrencyValue(f64::from_str(s)?))
    }
}

impl Deref for CurrencyValue {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
