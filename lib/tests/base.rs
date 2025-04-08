mod shared;
use shared::{get_invalid_server, get_server};

#[tokio::test]
async fn local_api_is_available() {
    let (server, _container) = get_server().await;
    assert!(
        server.is_server_available().await,
        "\x1b[1m \nIMPORTANT: Please ensure that there is a local Frankfurter API running.\n \x1b[0m"
    );
}

#[tokio::test]
async fn endpoint_currencies() {
    let (server, _container) = get_server().await;
    let res = server.currencies(Default::default()).await.unwrap();
    assert!(res.0.len() > 10);

    // ERROR RESPONSE FROM API
    let server = get_invalid_server().await;
    assert!(server.currencies(Default::default()).await.is_err())
}
