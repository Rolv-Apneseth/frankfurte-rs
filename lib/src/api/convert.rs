use std::borrow::Cow;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::{build_base_query_params, ServerClientRequest};
use crate::{
    data::{Currency, CurrencyValue, CurrencyValueMap},
    error::Error,
};

/// Response for fetching the latest exchange rates.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Base currency exchanged FROM.
    pub base: Currency,
    /// Amount of the base currency which was exchanged.
    pub amount: CurrencyValue,
    /// Date of the exchange rate used.
    pub date: NaiveDate,
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
    pub date: Option<NaiveDate>,
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
    pub fn with_date(mut self, date: NaiveDate) -> Self {
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
        if let Some(targets) = &self.targets {
            // Check against the default value too as passing targets which include the default (EUR),
            // will still cause an error to be returned from the API
            let base = self.base.clone().unwrap_or_default();
            if targets.contains(&base) {
                return Err(Error::RequestTargetsIncludeBase {
                    base,
                    targets: targets.clone(),
                });
            }
        }

        Ok(())
    }

    fn build_query_params(&self) -> super::QueryParams {
        build_base_query_params(&self.amount, &self.base, &self.targets)
    }
}
