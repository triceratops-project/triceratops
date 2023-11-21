use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router, Server};

pub async fn start() {
    let socket_address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let router = Router::new().route("/", get(handler));

    Server::bind(&socket_address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
