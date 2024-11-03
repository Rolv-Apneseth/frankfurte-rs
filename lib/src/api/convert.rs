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

impl ServerClientRequest for Request {
    /// Get the endpoint for fetching exchange rates for a specific date.
    fn get_url(&self) -> Cow<'static, str> {
        match self.date {
            Some(date) => format!("{date}").into(),
            None => "latest".into(),
        }
    }

    fn ensure_valid(&self) -> crate::error::Result<()> {
        if let (Some(targets), Some(base)) = (&self.targets, &self.base) {
            if targets.contains(base) {
                return Err(Error::RequestTargetsIncludeBase {
                    base: base.clone(),
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
