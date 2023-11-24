use axum::{routing::get, Router};

pub mod server;
pub mod servers;

pub fn router() -> Router<()> {
    let router = Router::new()
        .route("/", get(servers::handler))
        .route("/:server_id", get(server::handler));
    return router;
}
