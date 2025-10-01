use std::{fmt::Display, ops::Deref, str::FromStr};

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::Error;

/// Wrapper around a [`chronos::NaiveDate`], representing a valid date to fetch exchange rates for.
///
/// This wrapper ensures that values are limited by the min and max dates defined by [`ValidDate::min`] and [`ValidDate::max`]
///
/// # Example
/// ```
/// # use lib_frankfurter::ValidDate;
/// use chrono::{Days, Local, NaiveDate};
///
/// assert!(ValidDate::try_from(NaiveDate::from_ymd_opt(2001, 1, 1).unwrap()).is_ok());
/// assert!(ValidDate::try_from(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).is_ok());
///
/// assert!(ValidDate::try_from(NaiveDate::from_ymd_opt(0, 1, 1).unwrap()).is_err());
/// assert!(ValidDate::try_from(NaiveDate::from_ymd_opt(3005, 1, 1).unwrap()).is_err());
///
/// let tomorrow = Local::now().date_naive().checked_add_days(Days::new(1)).unwrap();
/// assert!(*ValidDate::max() < tomorrow);
/// assert!(*ValidDate::min() > NaiveDate::from_ymd_opt(1999, 1, 3).unwrap());
/// ```
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Deserialize, Serialize, Eq, Ord)]
pub struct ValidDate(NaiveDate);

impl ValidDate {
    const MIN: Self = ValidDate(NaiveDate::from_ymd_opt(1999, 1, 4).unwrap());

    pub fn min() -> Self {
        Self::MIN
    }
    pub fn max() -> Self {
        ValidDate(Local::now().date_naive())
    }

    fn is_valid_date(value: NaiveDate) -> bool {
        value >= *Self::min() && value <= *Self::max()
    }
}

impl TryFrom<NaiveDate> for ValidDate {
    type Error = Error;
    fn try_from(value: NaiveDate) -> std::result::Result<Self, Self::Error> {
        if Self::is_valid_date(value) {
            Ok(ValidDate(value))
        } else {
            Err(Error::InvalidDate(value.to_string()))
        }
    }
}

impl FromStr for ValidDate {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        ValidDate::try_from(NaiveDate::from_str(s).map_err(|_| Error::InvalidDate(s.to_owned()))?)
    }
}

impl Display for ValidDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ValidDate {
    fn default() -> Self {
        Self::min()
    }
}

impl Deref for ValidDate {
    type Target = NaiveDate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Days};
    use proptest::proptest;

    use super::*;

    #[test]
    fn test_date_validity_checked_on_creation() {
        assert_eq!(
            ValidDate::is_valid_date(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
            ValidDate::try_from(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()).is_ok()
        );
        assert_eq!(
            ValidDate::is_valid_date(NaiveDate::from_ymd_opt(0, 1, 1).unwrap()),
            ValidDate::try_from(NaiveDate::from_ymd_opt(0, 1, 1).unwrap()).is_ok()
        );

        assert_eq!(
            ValidDate::is_valid_date(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
            ValidDate::from_str("2000-01-01").is_ok()
        );
        assert_eq!(
            ValidDate::is_valid_date(NaiveDate::from_ymd_opt(0, 1, 1).unwrap()),
            ValidDate::from_str("0000-01-01").is_ok()
        );
        assert!(ValidDate::from_str("abc").is_err());
    }

    #[test]
    fn test_is_valid_date() {
        assert!(ValidDate::is_valid_date(*ValidDate::min()));
        assert!(ValidDate::is_valid_date(*ValidDate::max()));

        let lt_min = ValidDate::min().checked_sub_days(Days::new(1)).unwrap();
        assert!(!ValidDate::is_valid_date(lt_min));

        let gt_max = ValidDate::max().checked_add_days(Days::new(1)).unwrap();
        assert!(!ValidDate::is_valid_date(gt_max));

        // Other invalid values
        for (y, m, d) in [(0, 1, 1), (3005, 1, 1), (1999, 1, 1)] {
            let date = NaiveDate::from_ymd_opt(y, m, d).unwrap();
            assert!(!ValidDate::is_valid_date(date));
        }
    }

    proptest! {
        #[test]
        fn test_is_valid_date_props(y in ValidDate::min().year() + 1..ValidDate::max().year(), m in 1u32..13, d in 1u32..32) {
            if let Some(date) = NaiveDate::from_ymd_opt(y, m, d) {
                assert!(ValidDate::is_valid_date(date));
            } else {
                assert!(ValidDate::from_str(&format!("{:04}-{:02}-{:02}", y, m, d)).is_err());
            };
        }
    }
}
