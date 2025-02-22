use std::{collections::BTreeMap, fmt::Display, ops::Deref, str::FromStr};

use fast_float_compare::Float;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantNames};

use crate::error::Error;

/// A map of [`Currency`] to their respective [`CurrencyValue`], sorted by the currency code keys.
///
/// This represents a JSON response from the server outlining exchange rates for different currency
/// ISO 4217 codes.
pub(crate) type CurrencyValueMap = BTreeMap<Currency, CurrencyValue>;

// CURRENCY ----------------------------------------------------------------------------------------
/// Possible currency codes (ISO 4217) returned by the `Frankfurter` API.
///
/// [`Currency::Other`] provides support for historical currency codes, such as IEP (Irish pound).
///
/// # Example
/// ```
/// # use lib_frankfurter::Currency;
/// let australian_dollar = Currency::AUD;
/// let euro = Currency::EUR;
/// let spanish_peseta = Currency::Other("ESP".to_string());
/// ```
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

// CURRENCY VALUE ----------------------------------------------------------------------------------
/// Wrapper around an [`f64`], representing a valid currency value/amount.
///
/// This wrapper ensures that:
/// 1. Values are rounded to 2 decimal places when being displayed
/// 2. Values are limited by min and max values defined by [`CurrencyValue::MIN`] and [`CurrencyValue::MAX`], using comparisons provided by [`fast_float_compare`]
///
/// # Example
/// ```
/// # use lib_frankfurter::CurrencyValue;
/// assert!(CurrencyValue::try_from(0.2).is_ok_and(|f| *f == 0.2));
/// assert!(CurrencyValue::try_from(1.5).is_ok_and(|f| *f == 1.5));
/// assert!(CurrencyValue::try_from(100_000.4).is_ok_and(|f| *f == 100_000.4));
///
/// assert!(*CurrencyValue::MIN > 0.0);
/// assert!(*CurrencyValue::MAX < f64::MAX);
///
/// assert!(CurrencyValue::try_from(0.0).is_err());
/// assert!(CurrencyValue::try_from(-0.2).is_err());
/// assert!(CurrencyValue::try_from(f64::MAX).is_err());
/// ```
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct CurrencyValue(f64);

impl Display for CurrencyValue {
    // Limit to 2 decimal places
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl CurrencyValue {
    pub const MIN: Self = CurrencyValue(0.01);
    pub const MAX: Self = CurrencyValue(1_000_000_000_000.0);

    fn is_valid_currency_value(value: f64) -> bool {
        value.is_normal()
            && matches!(
                (Float::from_f64(*CurrencyValue::MIN), Float::from_f64(value)),
                (Some(min), Some(value)) if value >= min
            )
            && matches!(
                (Float::from_f64(*CurrencyValue::MAX), Float::from_f64(value)),
                (Some(max), Some(value)) if value <= max
            )
    }
}

impl TryFrom<f64> for CurrencyValue {
    type Error = Error;
    fn try_from(value: f64) -> std::result::Result<Self, Self::Error> {
        if Self::is_valid_currency_value(value) {
            Ok(CurrencyValue(value))
        } else {
            Err(Error::InvalidCurrencyValue(value.to_string()))
        }
    }
}

impl FromStr for CurrencyValue {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        CurrencyValue::try_from(
            f64::from_str(
                // Ignore potential thousand separators
                &s.replace([',', '_'], ""),
            )
            .map_err(|_| Error::InvalidCurrencyValue(s.to_string()))?,
        )
    }
}

impl Deref for CurrencyValue {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn test_currency_value_validity_checked_on_creation() {
        assert_eq!(
            CurrencyValue::is_valid_currency_value(1.0),
            CurrencyValue::try_from(1.0).is_ok()
        );
        assert_eq!(
            CurrencyValue::is_valid_currency_value(0.0),
            CurrencyValue::try_from(0.0).is_ok()
        );

        assert_eq!(
            CurrencyValue::is_valid_currency_value(1.0),
            CurrencyValue::from_str("1.0").is_ok()
        );
        assert_eq!(
            CurrencyValue::is_valid_currency_value(0.0),
            CurrencyValue::from_str("0.0").is_ok()
        );
        assert!(CurrencyValue::from_str("abc").is_err());
    }

    #[test]
    fn test_is_valid_currency_value() {
        assert!(CurrencyValue::is_valid_currency_value(*CurrencyValue::MIN));
        assert!(CurrencyValue::is_valid_currency_value(*CurrencyValue::MAX));

        assert!(!CurrencyValue::is_valid_currency_value(
            *CurrencyValue::MIN - 0.0001
        ));
        assert!(!CurrencyValue::is_valid_currency_value(
            *CurrencyValue::MAX + 0.0001
        ));

        for val in [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MIN_POSITIVE,
        ] {
            assert!(!CurrencyValue::is_valid_currency_value(val));
        }
    }

    proptest! {
        #[test]
        fn test_is_valid_currency_value_props(value in *CurrencyValue::MIN..=*CurrencyValue::MAX) {
            assert!(CurrencyValue::is_valid_currency_value(value));
        }
    }

    #[test]
    fn test_currency_value_ignores_separators() {
        let thousand = CurrencyValue(1_000.0);
        let million = CurrencyValue(1_000_000.0);

        assert_eq!(thousand, CurrencyValue::from_str("1,000").unwrap());
        assert_eq!(thousand, CurrencyValue::from_str("1_000").unwrap());

        assert_eq!(million, CurrencyValue::from_str("1,000,000").unwrap());
        assert_eq!(million, CurrencyValue::from_str("1_000_000").unwrap());

        assert_eq!(thousand, CurrencyValue::from_str("1,0,0,0,,").unwrap());
        assert_eq!(
            million,
            CurrencyValue::from_str("1,0,0,0,,_0_0_0_").unwrap()
        );

        assert!(CurrencyValue::from_str("0,0").is_err());
        assert!(CurrencyValue::from_str("0.0,0_1").is_err());
    }
}
