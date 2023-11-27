use dotenvy::dotenv;
use std::net::SocketAddr;

mod middleware;
mod routes;
mod state;

pub async fn start() {
    dotenv().ok();

    let socket_address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let router = routes::route().await;

    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
