extern crate reqwest;
extern crate serde_json;
extern crate tokio;
extern crate url;

use chrono::NaiveDate;
use lib_frankfurter::api::{convert, currencies, period, ServerClient};
use url::Url;

#[tokio::main]
async fn main() {
    let server_client: ServerClient = Url::parse("http://localhost:8080")
        .map(ServerClient::new)
        .unwrap_or_default();

    // Fetch currencies - this request does not accept any additional options
    match server_client
        .currencies(currencies::Request::default())
        .await
    {
        Ok(resp) => {
            println!("{}", serde_json::to_string_pretty(&resp).unwrap())
        }
        Err(e) => panic!("{e:?}"),
    }

    // Fetch latest exchange rates - for available, options check out [`convert::Request`]
    match server_client.convert(convert::Request::default()).await {
        Ok(resp) => {
            println!("{}", serde_json::to_string_pretty(&resp).unwrap())
        }
        Err(e) => panic!("{e:?}"),
    };

    // Fetch exchange rates from 01/01/2024 to the present date - for available options, check out [`period::Request`]
    match server_client
        .period(period::Request {
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => {
            println!("{}", serde_json::to_string_pretty(&resp).unwrap());
        }
        Err(e) => panic!("{e:?}"),
    };
}
