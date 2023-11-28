use axum::http::{header, Method};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod middleware;
mod routes;
mod state;

pub async fn start() {
    dotenv().ok();

    let socket_address = SocketAddr::from(([127, 0, 0, 1], 8080));

    #[cfg(not(debug_assertions))]
    compile_error!("REMOVE THE DANGEROUS CORS SHIT, CSRF AND ALL!?");

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_origin(AllowOrigin::mirror_request());

    let router = routes::route().await.layer(cors);

    let listener = TcpListener::bind(socket_address).await.unwrap();
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
