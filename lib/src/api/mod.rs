//! Bindings to the Frankfurter API
//!
//! The current bindings were generated using the
//! [HTTP API documentation](https://frankfurter.dev/).

pub mod convert;
pub mod currencies;
pub mod period;
mod shared;

use std::borrow::Cow;

use shared::*;
use url::Url;

use crate::error::Result;

/// A HTTP client for making requests to a Frankfurter API.
#[derive(Debug)]
pub struct ServerClient {
    url: Url,
    /// Inner client to perform HTTP requests.
    client: reqwest::Client,
}

impl Default for ServerClient {
    fn default() -> Self {
        Self {
            url: Url::parse("https://api.frankfurter.app")
                .expect("Invalid fallback Frankfurter API URL"),
            client: Default::default(),
        }
    }
}

impl ServerClient {
    pub fn new(frankfurter_api_url: Url) -> Self {
        Self {
            url: frankfurter_api_url,
            client: Default::default(),
        }
    }

    /// Consumes an existing [`ServerClient`] and returns one with the given [`reqwest::Client`].
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    /// Construct an HTTP URL base on the current hostname, optional port,
    /// and provided endpoint.
    #[inline]
    #[must_use]
    fn url(&self, endpoint: &str) -> String {
        format!("{}{endpoint}", self.url.as_str())
    }

    /// Makes a basic request to the API and returns true in the event of a successful response.
    ///
    /// Useful for a simple check that the API is up and successfully responding to requests.
    pub async fn is_server_available(&self) -> bool {
        self.client
            .get(self.url(""))
            .send()
            .await
            .is_ok_and(|r| r.status().is_success())
    }

    /// Request exchange rates for a specific date (latest by default).
    pub async fn convert(&self, req: convert::Request) -> Result<convert::Response> {
        let (url, query_params) = req.setup()?;

        self.client
            .get(self.url(&url))
            .query(&query_params)
            .send()
            .await?
            .json::<convert::Response>()
            .await
            .map_err(Into::into)
    }

    /// Request a time series of historical exchange rates.
    pub async fn period(&self, req: period::Request) -> Result<period::Response> {
        let (url, query_params) = req.setup()?;

        self.client
            .get(self.url(&url))
            .query(&query_params)
            .send()
            .await?
            .json::<period::Response>()
            .await
            .map_err(Into::into)
    }

    /// Request the supported currency codes and their full names
    pub async fn currencies(&self, req: currencies::Request) -> Result<currencies::Response> {
        let (url, _) = req.setup()?;

        self.client
            .get(self.url(&url))
            .send()
            .await?
            .json::<currencies::Response>()
            .await
            .map_err(Into::into)
    }
}

pub type EndpointUrl = Cow<'static, str>;
pub type QueryParams = Vec<(&'static str, String)>;

/// Utility trait to provide a common interface for server client requests.
pub trait ServerClientRequest {
    fn get_url(&self) -> EndpointUrl;
    fn ensure_valid(&self) -> Result<()>;

    fn build_query_params(&self) -> QueryParams {
        Vec::new()
    }

    fn setup(&self) -> Result<(EndpointUrl, QueryParams)> {
        self.ensure_valid()?;
        let url = self.get_url();
        let query_params = self.build_query_params();
        Ok((url, query_params))
    }
}


}
