use axum::Server;
use dotenvy::dotenv;
use std::net::SocketAddr;

pub async fn start() {
    dotenv().ok();

    let socket_address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let router = super::routes::route().await;

    Server::try_bind(&socket_address)
        .unwrap()
        .serve(router.into_make_service())
        .await
        .unwrap();
}
