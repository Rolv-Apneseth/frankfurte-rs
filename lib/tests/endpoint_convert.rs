mod shared;
use chrono::NaiveDate;
use lib_frankfurter::{
    api::convert,
    data::{Currency, CurrencyValue},
};
use pretty_assertions::assert_eq;
use shared::{get_invalid_server, get_server};

#[tokio::test]
async fn endpoint_convert() {
    let server = get_server();
    let make_request = |request: convert::Request| async { server.convert(request).await.unwrap() };

    // BASIC
    let res = make_request(Default::default()).await;
    assert_eq!(res.base, Currency::EUR);
    assert!(res.rates.len() > 10);
    assert_eq!(res.amount, CurrencyValue::from(1.0));

    // BASE CURRENCY AND AMOUNT
    let base = Currency::USD;
    let amount = CurrencyValue::from(4.0);
    let res = make_request(
        convert::Request::default()
            .with_base(base.clone())
            .with_amount(amount),
    )
    .await;
    assert_eq!(res.base, base);
    assert_eq!(res.amount, amount);

    // TARGETS
    let targets = vec![Currency::AUD, Currency::DKK, Currency::ZAR];
    let res = make_request(convert::Request::default().with_targets(targets.clone())).await;
    assert_eq!(res.rates.len(), targets.len());

    // DATE
    let date = NaiveDate::from_ymd_opt(2024, 8, 20).unwrap();
    let res = make_request(convert::Request::default().with_date(date)).await;
    assert_eq!(res.date, date);

    // ERROR RESPONSE FROM API
    let server = get_invalid_server();
    assert!(server.convert(Default::default()).await.is_err())
}
