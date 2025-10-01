use crate::{
    data::{Currency, CurrencyValue},
    error::{Error, Result},
};

/// Shared validation checks between [`super::convert::Request`] and [`super::period::Request`].
pub(super) fn base_ensure_valid(
    base: &Option<Currency>,
    targets: &Option<Vec<Currency>>,
) -> Result<()> {
    if let Some(targets) = targets {
        // Check against the default value too as passing targets which include the default (EUR),
        // will still cause an error to be returned from the API
        let base = base.clone().unwrap_or_default();
        if targets.contains(&base) {
            return Err(Error::RequestTargetsIncludeBase {
                base,
                targets: targets.clone(),
            });
        }
    }

    Ok(())
}

/// Shared query parameters between [`super::convert::Request`] and [`super::period::Request`].
pub(super) fn base_build_query_params(
    amount: &Option<CurrencyValue>,
    base: &Option<Currency>,
    targets: &Option<Vec<Currency>>,
) -> Vec<(&'static str, String)> {
    let mut query_params = vec![];

    if let Some(a) = amount {
        query_params.push(("amount", format!("{a:.2}")));
    };

    if let Some(b) = base {
        query_params.push(("base", b.to_string()));
    };

    if let Some(t) = targets {
        // An empty string for the target/symbol would lead to an error being returned from the API,
        // so skip if targets is empty
        if !t.is_empty() {
            query_params.push((
                "symbols",
                t.iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            ))
        }
    };

    query_params
}

#[cfg(test)]
mod tests_shared {
    use super::*;
    use crate::api::test_utils::dbg_err;

    #[test]
    fn test_base_ensure_valid() {
        // DEFAULT
        assert!(base_ensure_valid(&None, &None).inspect_err(dbg_err).is_ok());

        // VALID TARGETS
        assert!(
            base_ensure_valid(&None, &Some(vec![Currency::USD, Currency::AUD]))
                .inspect_err(dbg_err)
                .is_ok()
        );

        // INVALID TARGETS
        assert!(
            base_ensure_valid(
                &Some(Currency::USD),
                &Some(vec![Currency::USD, Currency::AUD])
            )
            .is_err()
        );

        // Check against default (EUR)
        assert!(base_ensure_valid(&None, &Some(vec![Currency::EUR, Currency::AUD])).is_err());
    }

    #[test]
    fn test_base_build_query_params() {
        // DEFAULT
        assert_eq!(base_build_query_params(&None, &None, &None), vec![]);

        // INDIVIDUAL
        assert_eq!(
            base_build_query_params(&Some(CurrencyValue::try_from(10.0).unwrap()), &None, &None),
            vec![("amount", String::from("10.00"))]
        );
        assert_eq!(
            base_build_query_params(&None, &Some(Currency::AUD), &None),
            vec![("base", String::from("AUD"))]
        );
        assert_eq!(
            base_build_query_params(&None, &None, &Some(vec![Currency::CAD, Currency::ZAR])),
            vec![("symbols", String::from("CAD,ZAR"))]
        );

        // COMBOS
        assert_eq!(
            base_build_query_params(
                &Some(CurrencyValue::try_from(1000000.0).unwrap()),
                &Some(Currency::USD),
                &Some(vec![Currency::CNY, Currency::CZK, Currency::IDR])
            ),
            vec![
                ("amount", String::from("1000000.00")),
                ("base", String::from("USD")),
                ("symbols", String::from("CNY,CZK,IDR")),
            ]
        );
        assert_eq!(
            base_build_query_params(
                &None,
                &Some(Currency::USD),
                &Some(vec![Currency::CNY, Currency::CZK, Currency::IDR])
            ),
            vec![
                ("base", String::from("USD")),
                ("symbols", String::from("CNY,CZK,IDR")),
            ]
        );
        assert_eq!(
            base_build_query_params(
                &Some(CurrencyValue::MIN),
                &None,
                &Some(vec![Currency::CNY, Currency::CZK, Currency::IDR])
            ),
            vec![
                ("amount", (*CurrencyValue::MIN).to_string()),
                ("symbols", String::from("CNY,CZK,IDR")),
            ]
        );
        assert_eq!(
            base_build_query_params(
                &Some(CurrencyValue::try_from(1000000.0).unwrap()),
                &Some(Currency::USD),
                &None
            ),
            vec![
                ("amount", String::from("1000000.00")),
                ("base", String::from("USD")),
            ]
        );
    }
}
