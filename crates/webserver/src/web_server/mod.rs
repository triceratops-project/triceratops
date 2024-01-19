use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod middleware;
mod routes;
mod state;

pub async fn start() {
    dotenv().ok();

    let socket_port = std::env::var("PORT").expect("PORT must be set");
    let socket_address = SocketAddr::from(([127, 0, 0, 1], socket_port.parse::<u16>().unwrap()));

    let router = routes::route().await;

    let listener = TcpListener::bind(socket_address).await.unwrap();
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
