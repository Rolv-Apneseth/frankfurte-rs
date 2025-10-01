//! [`Request`] and [`Response`] types for requesting exchange rates for a specific date (latest by default).

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{ServerClientRequest, base_build_query_params, base_ensure_valid};
use crate::data::{Currency, CurrencyValue, CurrencyValueMap, ValidDate};

/// Response for fetching the latest exchange rates.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Base currency exchanged FROM.
    pub base: Currency,
    /// Amount of the base currency which was exchanged.
    pub amount: CurrencyValue,
    /// Date of the exchange rate used.
    pub date: ValidDate,
    /// Map of the currencies to their respective exchanged values.
    pub rates: CurrencyValueMap,
}

/// Request query parameters for fetching the latest exchange rates.
#[derive(Clone, PartialEq, Debug, Serialize, Default)]
pub struct Request {
    /// Base currency to be exchanged FROM.
    pub base: Option<Currency>,
    /// Currencies to exchange the base currency TO.
    pub targets: Option<Vec<Currency>>,
    /// Amount of the base currency to be exchanged.
    pub amount: Option<CurrencyValue>,
    /// Date of the exchange rate(s) to be used.
    pub date: Option<ValidDate>,
}

impl Request {
    /// Consumes the [`Request`] and returns a new one with the given base.
    pub fn with_base(mut self, base: Currency) -> Self {
        self.base = Some(base);
        self
    }

    /// Consumes the [`Request`] and returns a new one with the given targets.
    pub fn with_targets(mut self, targets: Vec<Currency>) -> Self {
        self.targets = Some(targets);
        self
    }

    /// Consumes the [`Request`] and returns a new one with the given amount.
    pub fn with_amount(mut self, amount: CurrencyValue) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Consumes the [`Request`] and returns a new one with the given date.
    pub fn with_date(mut self, date: ValidDate) -> Self {
        self.date = Some(date);
        self
    }
}

impl ServerClientRequest for Request {
    /// Get the endpoint for fetching exchange rates for a specific date.
    fn get_url(&self) -> Cow<'static, str> {
        match self.date {
            Some(date) => format!("{date}").into(),
            None => "latest".into(),
        }
    }

    fn ensure_valid(&self) -> crate::error::Result<()> {
        base_ensure_valid(&self.base, &self.targets)
    }

    fn build_query_params(&self) -> super::QueryParams {
        base_build_query_params(&self.amount, &self.base, &self.targets)
    }
}

#[cfg(test)]
mod tests_convert {
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::api::test_utils::dbg_err;

    #[test]
    fn test_get_url() {
        assert_eq!(Request::default().get_url(), "latest");

        let date = ValidDate::try_from(NaiveDate::from_ymd_opt(2000, 7, 2).unwrap()).unwrap();
        assert_eq!(
            Request::default().with_date(date).get_url(),
            format!("{date}")
        );
    }

    #[test]
    fn test_ensure_valid() {
        // Check that [`super::base_ensure_valid`] is being called
        assert!(
            Request::default()
                .with_base(Currency::EUR)
                .with_targets(vec![Currency::EUR, Currency::USD])
                .ensure_valid()
                .is_err()
        );

        // VALID DATE
        assert!(
            Request::default()
                .with_date(ValidDate::max())
                .ensure_valid()
                .inspect_err(dbg_err)
                .is_ok()
        );

        // Weekend - will just use the closest date with data
        assert!(
            Request::default()
                .with_date(
                    ValidDate::try_from(NaiveDate::from_ymd_opt(2024, 2, 3).unwrap()).unwrap()
                )
                .ensure_valid()
                .inspect_err(dbg_err)
                .is_ok()
        );
    }
}
