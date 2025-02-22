mod shared;
use std::str::FromStr;

use chrono::{Datelike, NaiveDate};
use lib_frankfurter::{api::period, Currency, CurrencyValue, ValidDate};
use pretty_assertions::assert_eq;
use shared::{get_invalid_server, get_server};

#[tokio::test]
async fn endpoint_period() {
    let server = get_server();
    let make_request = |request: period::Request| async { server.period(request).await.unwrap() };

    // BASIC
    let res = make_request(Default::default()).await;
    assert_eq!(res.start_date, ValidDate::min());
    assert_eq!(res.base, Currency::EUR);
    // Shouldn't include the base currency and the fallback currency
    assert!(res.rates.last_key_value().unwrap().1.len() > 10);
    assert_eq!(res.amount, CurrencyValue::try_from(1.0).unwrap());
    assert!(res.rates.len() > 1000);

    // BASE CURRENCY AND AMOUNT
    let base = Currency::KRW;
    let amount = CurrencyValue::try_from(10.0).unwrap();
    let res = make_request(
        period::Request::default()
            .with_base(base.clone())
            .with_amount(amount),
    )
    .await;
    assert_eq!(res.base, base);
    assert_eq!(res.amount, amount);

    // TARGETS
    let targets = vec![Currency::CHF, Currency::CAD, Currency::CNY];
    let res = make_request(period::Request::default().with_targets(targets.clone())).await;
    assert_eq!(res.rates.last_key_value().unwrap().1.len(), targets.len());

    // STARTING DATE
    let start_date = ValidDate::try_from(NaiveDate::from_ymd_opt(2020, 10, 5).unwrap()).unwrap();
    let res = make_request(period::Request::default().with_start_date(start_date)).await;
    assert_eq!(res.start_date, start_date);
    assert!(res.rates.len() > 200);

    // STARTING AND END DATES
    let start_date = ValidDate::from_str("2024-10-07").unwrap();
    let end_date = ValidDate::from_str("2024-10-11").unwrap();
    let res = make_request(
        period::Request::default()
            .with_start_date(start_date)
            .with_end_date(end_date),
    )
    .await;
    assert_eq!(res.start_date, start_date);
    assert_eq!(res.end_date.unwrap(), end_date);
    assert_eq!(
        res.rates.len(),
        // Start -> end date (inclusive)
        (end_date.num_days_from_ce() - start_date.num_days_from_ce() + 1) as usize
    );

    // ERROR RESPONSE FROM API
    let server = get_invalid_server();
    assert!(server.period(Default::default()).await.is_err())
}
