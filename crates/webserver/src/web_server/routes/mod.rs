use crate::{config::TriceratopsConfig, state::InternalAppState};
use axum::Router;
use error_stack::{Context, Result, ResultExt};
use std::{fmt, sync::Arc};

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

    let api_router = Router::new().nest("/api", api::route(&app_state));

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
