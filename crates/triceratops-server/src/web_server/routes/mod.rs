use crate::{config::TriceratopsConfig, state::InternalAppState};
use axum::Router;
use error_stack::{Context, Result, ResultExt};
use std::{fmt, sync::Arc};
use tower_http::cors::CorsLayer;

mod api;
mod auth;
mod servers;
mod spa;
mod users;

#[derive(Debug)]
pub struct RouterError;

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Router error")
    }
}

impl Context for RouterError {}

pub async fn route(config: TriceratopsConfig) -> Result<Router, RouterError> {
    let app_state_raw = InternalAppState::new(config.clone())
        .await
        .attach_printable("Failed to assemble app state")
        .change_context(RouterError)?;

    let app_state = Arc::new(app_state_raw);

    let cord_origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://localhost:5173".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_private_network(true)
        .allow_origin(cord_origins);

    let api_router = Router::new().nest("/api", api::route(&app_state).layer(cors));

    let spa_router = spa::router();

    let router = match *config.web_server.headless() {
        Some(true) => Router::new().merge(api_router).with_state(app_state),
        Some(false) | None => Router::new()
            .merge(api_router)
            .merge(spa_router)
            .with_state(app_state),
    };

    Ok(router)
}
