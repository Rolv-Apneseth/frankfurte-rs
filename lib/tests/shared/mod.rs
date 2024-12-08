use std::sync::LazyLock;

use lib_frankfurter::api;
use url::Url;

/// URL for locally hosted API
static URL: LazyLock<Url> = LazyLock::new(|| Url::parse("http://localhost:8080").unwrap());
static INVALID_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("http://localhost:8080/invalid").unwrap());

pub fn get_server() -> api::ServerClient {
    api::ServerClient::new(URL.clone())
}

pub fn get_invalid_server() -> api::ServerClient {
    api::ServerClient::new(INVALID_URL.clone())
}
