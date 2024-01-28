use dotenvy::dotenv;
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
    if dotenv().is_err() {
        println!("Failed to read .env");
        println!(".env sucks and I'm making a binary format config");
    }

    let socket_port_env = std::env::var("PORT")
        .map_err(Report::from)
        .attach_printable("Failed to read variable PORT")
        .change_context(WebServerError)?;

    let socket_port = socket_port_env
        .parse::<u16>()
        .map_err(Report::from)
        .attach_printable_lazy(|| format!("Failed to convert {} into port (u16)", socket_port_env))
        .change_context(WebServerError)?;

    let socket_address = SocketAddr::from(([127, 0, 0, 1], socket_port));

    let router = routes::route()
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
    .await
    .map_err(Report::from)
    .attach_printable("Failed to build web server")
    .change_context(WebServerError)?;

    Ok(())
}
