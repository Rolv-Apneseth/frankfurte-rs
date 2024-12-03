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

use crate::error::{Error, Result};

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
    pub fn new(mut frankfurter_api_url: Url) -> Self {
        // Remove any number of trailing `/`
        while frankfurter_api_url.path().ends_with('/')
            && frankfurter_api_url.path_segments().unwrap().count() != 1
        {
            frankfurter_api_url.path_segments_mut().unwrap().pop();
        }

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

    /// Construct an HTTP URL with the base and the provided endpoint.
    #[inline]
    #[must_use]
    fn build_endpoint(&self, endpoint: &str) -> Url {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .expect("Couldn't get path segments")
            .push(&endpoint.replace('/', ""));
        url
    }

    /// Makes a basic request to the root of the API and returns true in the event of a successful response.
    ///
    /// Useful for a simple check that the API is up and successfully responding to requests.
    pub async fn is_server_available(&self) -> bool {
        let mut base_url = self.url.clone();
        base_url.set_path("");

        self.client
            .get(base_url)
            .send()
            .await
            .is_ok_and(|r| r.status().is_success())
    }

    /// Internal method for handling `GET` requests.
    async fn get<Resp: for<'de> serde::Deserialize<'de>>(
        &self,
        req: impl ServerClientRequest,
    ) -> Result<Resp> {
        let (endpoint, params) = req.setup()?;
        let resp = self
            .client
            .get(self.build_endpoint(&endpoint))
            .query(&params)
            .send()
            .await?;

        // Return an error in the case of a response with an error status code from the API
        if let Err(err) = resp.error_for_status_ref() {
            return Err(Error::InvalidResponse {
                status: err.status().expect("Couldn't get status from response"),
                body: resp.text().await.unwrap_or_default(),
                url: self.build_endpoint(&endpoint).to_string(),
            });
        };

        resp.json::<Resp>().await.map_err(Into::into)
    }

    /// Request exchange rates for a specific date (latest by default).
    pub async fn convert(&self, req: convert::Request) -> Result<convert::Response> {
        self.get::<convert::Response>(req).await
    }

    /// Request a time series of historical exchange rates.
    pub async fn period(&self, req: period::Request) -> Result<period::Response> {
        self.get::<period::Response>(req).await
    }

    /// Request the supported currency codes and their full names.
    pub async fn currencies(&self, req: currencies::Request) -> Result<currencies::Response> {
        self.get::<currencies::Response>(req).await
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

#[cfg(test)]
mod test_utils {
    use crate::error::Error;

    pub fn dbg_err(e: &Error) {
        dbg!(e);
    }
}
