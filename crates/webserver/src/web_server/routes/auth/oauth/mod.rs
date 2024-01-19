use crate::web_server::state::AppState;
use axum::{routing::get, Router};

mod discord;
mod services;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(services::handler))
        .nest("/discord", discord::router())
}
