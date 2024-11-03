use std::{borrow::Cow, cmp::Ordering, collections::BTreeMap};

use chrono::{Datelike, NaiveDate, TimeDelta, Timelike, Utc, Weekday};
use serde::{Deserialize, Serialize};

use super::{build_base_query_params, ServerClientRequest};
use crate::{
    data::{Currency, CurrencyValue, CurrencyValueMap},
    error::Error,
};

/// Response for fetching the latest exchange rates.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Response {
    /// The ISO 4217 code of the base currency being compared
    pub base: Currency,
    /// Amount of the base currency being compared
    pub amount: CurrencyValue,
    /// Start date representation in the form `yyyy-mm-dd`
    pub start_date: NaiveDate,
    /// End date representation in the form `yyyy-mm-dd`
    pub end_date: Option<NaiveDate>,
    /// Map of dates to currency exchange maps
    pub rates: BTreeMap<NaiveDate, CurrencyValueMap>,
}

/// Request query parameters for fetching the latest exchange rates.
#[derive(Clone, PartialEq, Debug, Serialize, Default)]
pub struct Request {
    pub amount: Option<CurrencyValue>,
    pub base: Option<Currency>,
    pub targets: Option<Vec<Currency>>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

impl ServerClientRequest for Request {
    /// Get the endpoint for fetching exchange rates over a period of time.
    fn get_url(&self) -> Cow<'static, str> {
        match self.end_date {
            None => format!("{}..", self.start_date).into(),
            Some(end_date) => format!("{}..{}", self.start_date, end_date).into(),
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

        let mut latest = Utc::now();
        // Reduce day by 1 if it is still earlier than the earliest exchange rate fetch at 15:00
        if latest.time().hour() < 14 {
            latest -= TimeDelta::days(1);
        }

        if self.start_date > latest.date_naive() {
            return Err(Error::RequestLateStartDate(self.start_date));
        }

        if let Some(end_date) = self.end_date {
            match self.start_date.cmp(&end_date) {
                Ordering::Greater => {
                    return Err(Error::RequestEndDateBeforeStart {
                        start: self.start_date,
                        end: end_date,
                    })
                }
                Ordering::Equal | Ordering::Less => {
                    match (self.start_date.weekday(), end_date.weekday()) {
                        // Server returns error when asking for period which only spans a weekend
                        (Weekday::Sat, Weekday::Sat)
                        | (Weekday::Sun, Weekday::Sun)
                        | (Weekday::Sat, Weekday::Sun) => {
                            return Err(Error::RequestWeekendDates {
                                start: self.start_date,
                                end: end_date,
                            })
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    fn build_query_params(&self) -> super::QueryParams {
        build_base_query_params(&self.amount, &self.base, &self.targets)
    }
}
