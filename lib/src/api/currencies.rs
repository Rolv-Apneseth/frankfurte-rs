use std::{borrow::Cow, collections::BTreeMap};

use serde::{Deserialize, Serialize};

use super::ServerClientRequest;
use crate::data::Currency;

/// Response for fetching currencies
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response(pub BTreeMap<Currency, String>);

/// Request query parameters for fetching the available currencies to exchange between.
#[derive(Clone, PartialEq, Debug, Serialize, Default)]
pub struct Request {}

impl ServerClientRequest for Request {
    /// Get the endpoint for fetching the available currencies.
    fn get_url(&self) -> Cow<'static, str> {
        "currencies".into()
    }

    fn ensure_valid(&self) -> crate::error::Result<()> {
        Ok(())
    }
}
