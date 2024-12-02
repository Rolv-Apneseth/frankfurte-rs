use std::{borrow::Cow, collections::BTreeMap};

use chrono::{NaiveDate, TimeDelta, Timelike, Utc};
use serde::{Deserialize, Serialize};

use super::{base_build_query_params, base_ensure_valid, ServerClientRequest};
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

    /// Consumes the [`Request`] and returns a new one with the given start date.
    pub fn with_start_date(mut self, date: NaiveDate) -> Self {
        self.start_date = date;
        self
    }

    /// Consumes the [`Request`] and returns a new one with the given end date.
    pub fn with_end_date(mut self, date: NaiveDate) -> Self {
        self.end_date = Some(date);
        self
    }
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
        base_ensure_valid(&self.base, &self.targets)?;

        let mut latest = Utc::now();
        // Reduce day by 1 if it is still earlier than the earliest exchange rate fetch at 15:00
        if latest.time().hour() < 14 {
            latest -= TimeDelta::days(1);
        }

        if self.start_date > latest.date_naive() {
            return Err(Error::RequestLateStartDate(self.start_date));
        }

        if let Some(end_date) = self.end_date {
            if self.start_date.gt(&end_date) {
                return Err(Error::RequestEndDateBeforeStart {
                    start: self.start_date,
                    end: end_date,
                });
            }
        }

        Ok(())
    }

    fn build_query_params(&self) -> super::QueryParams {
        base_build_query_params(&self.amount, &self.base, &self.targets)
    }
}

#[cfg(test)]
mod tests_period {
    use chrono::Days;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::api::test_utils::dbg_err;

    #[test]
    fn get_url() {
        assert_eq!(
            Request::default().get_url(),
            format!("{}..", NaiveDate::default())
        );

        let date = NaiveDate::from_ymd_opt(2000, 7, 2).unwrap();
        assert_eq!(
            Request::default().with_start_date(date).get_url(),
            format!("{date}..")
        );

        let date = NaiveDate::from_ymd_opt(2020, 8, 9).unwrap();
        assert_eq!(
            Request::default().with_end_date(date).get_url(),
            format!("{}..{date}", NaiveDate::default())
        );

        let start_date = NaiveDate::from_ymd_opt(2020, 8, 9).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 10, 9).unwrap();
        assert_eq!(
            Request::default()
                .with_start_date(start_date)
                .with_end_date(end_date)
                .get_url(),
            format!("{start_date}..{end_date}")
        );
    }

    #[test]
    fn ensure_valid() {
        // Check that [`super::base_ensure_valid`] is being called
        assert!(Request::default()
            .with_base(Currency::EUR)
            .with_targets(vec![Currency::EUR, Currency::USD])
            .ensure_valid()
            .is_err());

        // VALID START DATE
        assert!(Request::default()
            .with_start_date(NaiveDate::default())
            .ensure_valid()
            .inspect_err(dbg_err)
            .is_ok());

        // INVALID START DATE
        let tomorrow = Utc::now()
            .checked_add_days(Days::new(1))
            .unwrap()
            .date_naive();
        assert!(Request::default()
            .with_start_date(tomorrow)
            .ensure_valid()
            .is_err());

        // VALID END DATES
        assert!(Request::default()
            .with_end_date(NaiveDate::from_ymd_opt(2000, 2, 4).unwrap())
            .ensure_valid()
            .inspect_err(dbg_err)
            .is_ok());

        // Not quite weekend only - Friday-Sun
        assert!(Request::default()
            .with_start_date(NaiveDate::from_ymd_opt(2024, 8, 2).unwrap())
            .with_end_date(NaiveDate::from_ymd_opt(2024, 8, 4).unwrap())
            .ensure_valid()
            .inspect_err(dbg_err)
            .is_ok());
        // Weekend only - Sat-Sun
        assert!(Request::default()
            .with_start_date(NaiveDate::from_ymd_opt(2024, 8, 3).unwrap())
            .with_end_date(NaiveDate::from_ymd_opt(2024, 8, 4).unwrap())
            .ensure_valid()
            .inspect_err(dbg_err)
            .is_ok());
        // Weekend only - Sat-Sat
        assert!(Request::default()
            .with_start_date(NaiveDate::from_ymd_opt(2024, 1, 13).unwrap())
            .with_end_date(NaiveDate::from_ymd_opt(2024, 1, 13).unwrap())
            .ensure_valid()
            .inspect_err(dbg_err)
            .is_ok());
        // Weekend only - Sun-Sun
        assert!(Request::default()
            .with_start_date(NaiveDate::from_ymd_opt(2024, 6, 23).unwrap())
            .with_end_date(NaiveDate::from_ymd_opt(2024, 6, 23).unwrap())
            .ensure_valid()
            .inspect_err(dbg_err)
            .is_ok());

        // INVALID END DATE
        assert!(Request::default()
            .with_end_date(NaiveDate::default().checked_sub_days(Days::new(1)).unwrap())
            .ensure_valid()
            .is_err());
    }
}
