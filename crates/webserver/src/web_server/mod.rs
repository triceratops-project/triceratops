use crate::config::TriceratopsConfig;
use error_stack::{Context, Report, Result, ResultExt};
use std::{fmt, net::SocketAddr};
use tokio::net::TcpListener;

mod middleware;
mod routes;

#[derive(Debug)]
pub struct WebServerError;

impl fmt::Display for WebServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Web server error")
    }
}

impl Context for WebServerError {}

pub async fn start() -> Result<(), WebServerError> {
    let config = TriceratopsConfig::load()
        .attach_printable("Failed to load config")
        .change_context(WebServerError)?;

    let ip = *config.web_server().bind_address();

    let port = *config.web_server().port();

    let socket_address = SocketAddr::new(ip, port);

    let router = routes::route(config)
        .await
        .attach_printable("Failed to build router")
        .change_context(WebServerError)?;

    let listener = TcpListener::bind(socket_address)
        .await
        .map_err(Report::from)
        .attach_printable_lazy(|| format!("Failed to bind to {}", socket_address))
        .attach_printable("Is Triceratops already running?")
        .change_context(WebServerError)?;

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    // .with_graceful_shutdown(axum_shutdown())
    .await
    .map_err(Report::from)
    .attach_printable("Failed to build web server")
    .change_context(WebServerError)?;

    Ok(())
}
