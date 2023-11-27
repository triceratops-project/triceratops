use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod middleware;
mod routes;
mod state;

pub async fn start() {
    dotenv().ok();

    let socket_address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let router = routes::route().await;

    let listener = TcpListener::bind(socket_address).await.unwrap();
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
