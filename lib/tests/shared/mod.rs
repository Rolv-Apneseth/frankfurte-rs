use std::time::Duration;

use testcontainers::{
    core::{wait::HealthWaitStrategy, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

use lib_frankfurter::api;
use url::Url;

async fn get_container() -> ContainerAsync<GenericImage> {
    GenericImage::new("lineofflight/frankfurter", "latest")
        .with_exposed_port(8080.tcp())
        .with_wait_for(WaitFor::Healthcheck(
            HealthWaitStrategy::new().with_poll_interval(Duration::from_secs(5)),
        ))
        .with_network("bridge")
        .start()
        .await
        .unwrap()
}

pub async fn get_server() -> (api::ServerClient, ContainerAsync<GenericImage>) {
    let container = get_container().await;

    let port = container
        .get_host_port_ipv4(8080.tcp())
        .await
        .expect("could not get host port");

    let host = container.get_host().await.unwrap();

    (
        api::ServerClient::new(Url::parse(&format!("http://{host}:{port}")).unwrap()),
        container,
    )
}

pub async fn get_invalid_server() -> api::ServerClient {
    api::ServerClient::new(Url::parse("http://localhost:0").unwrap())
}
