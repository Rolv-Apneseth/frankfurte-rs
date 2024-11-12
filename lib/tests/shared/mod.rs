use std::sync::LazyLock;

use lib_frankfurter::api;
use url::Url;

/// URL for locally hosted API
static URL: LazyLock<Url> = LazyLock::new(|| Url::parse("http://localhost:8080").unwrap());

pub fn get_server() -> api::ServerClient {
    api::ServerClient::new(URL.clone())
}
